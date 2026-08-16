//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1148/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1148<F: Float>(t14565: F, t14567: F, t1135: F, t3065: F, t2134: F, t14059: F, t14073: F, t14080: F, t14085: F, t14554: F, t14556: F, t14558: F, t14560: F, t14563: F) -> (F, F) {
    let t14568 = t14565 * t14567;
    let t14570 = t3065 * t1135;
    let t14571 = t2134 * t14570;
    let t14574 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14554 - t14556 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t14558 - t14560 / F::cast_from(192.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t14059 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14563 + t14568 / F::cast_from(96.0_f64) - t14571 / F::cast_from(96.0_f64) + t14073 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t14080 + t14085;
    (t14570, t14574)
}
