//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1035/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1035(t2158: f64, t2345: f64, t3240: f64, t3219: f64, t6366: f64, t6524: f64, t2343: f64, t3247: f64, t6204: f64, t6225: f64, t8844: f64, t8846: f64, t8853: f64, t8854: f64, t8858: f64, t8866: f64, t8871: f64, t8876: f64) -> (f64, f64, f64) {
    let t9353 = t2345 * t3240 * t2158;
    let t9358 = t6366 * t3219 * t6524;
    let t9362 = t8844 - t3247 * t9353 / 128.0_f64 - t8846 + 7.0_f64 / 288.0_f64 * t6204 - t8853 - 5.0_f64 / 384.0_f64 * t2343 * t9358 - 7.0_f64 / 2304.0_f64 * t6225 - t8854 + t8858 + t8866 + t8871 + t8876;
    (t9353, t9358, t9362)
}
