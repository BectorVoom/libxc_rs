//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 644/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk644<F: Float>(t1044: F, t2607: F, t1621: F, t1620: F, t1037: F, t2612: F, t3354: F, t643: F, t642: F, t639: F, t1643: F, t3351: F) -> (F, F, F, F, F, F, F, F) {
    let t3512 = t2607 * t1044;
    let t3513 = t1621 * t3512;
    let t3515 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1620 * t3513;
    let t3517 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2612 * t1037;
    let t3518 = t643 * t3354;
    let t3519 = t642 * t3518;
    let t3521 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t639 * t3519;
    let t3522 = t1643 * t3351;
    (t3512, t3513, t3515, t3517, t3518, t3519, t3521, t3522)
}
