//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 752/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk752<F: Float>(t1289: F, t387: F, t13: F, t1292: F, t30: F, t4510: F, t2704: F, t2718: F, t4518: F, t4521: F, t4524: F, t4529: F, t4531: F, t4533: F) -> (F, F) {
    let t4658 = F::cast_from(1.0_f64) / t1289 / t387;
    let t4659 = t13 * t4658;
    let t4661 = F::cast_from(1.0_f64) / t1292 / t30;
    let t4662 = t4510 * t4661;
    let t4663 = t4659 * t4662;
    let t4664 = F::cast_from(0.51725014705706168417e3_f64) * t4663;
    let t4673 = -F::cast_from(0.47063e1_f64) * t4518 + F::cast_from(0.31375333333333333334e1_f64) * t4521 - F::cast_from(0.36604555555555555556e1_f64) * t4524 - F::cast_from(0.16068111111111111111e1_f64) * t2704 + F::cast_from(0.28051666666666666666e0_f64) * t4529 - F::cast_from(0.56103333333333333332e0_f64) * t4531 - F::cast_from(0.6545388888888888889e0_f64) * t4533 - F::cast_from(0.46308888888888888888e0_f64) * t2718;
    (t4664, t4673)
}
