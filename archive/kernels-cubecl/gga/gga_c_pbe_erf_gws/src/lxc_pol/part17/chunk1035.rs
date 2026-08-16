//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1035/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1035<F: Float>(t2158: F, t2345: F, t3240: F, t3219: F, t6366: F, t6524: F, t2343: F, t3247: F, t6204: F, t6225: F, t8844: F, t8846: F, t8853: F, t8854: F, t8858: F, t8866: F, t8871: F, t8876: F) -> (F, F, F) {
    let t9353 = t2345 * t3240 * t2158;
    let t9358 = t6366 * t3219 * t6524;
    let t9362 = t8844 - t3247 * t9353 / F::cast_from(128.0_f64) - t8846 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t6204 - t8853 - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t2343 * t9358 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t6225 - t8854 + t8858 + t8866 + t8871 + t8876;
    (t9353, t9358, t9362)
}
