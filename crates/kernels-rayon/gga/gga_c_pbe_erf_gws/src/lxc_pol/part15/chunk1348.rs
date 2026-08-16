//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1348/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1348(t54716: f64, t4130: f64, t51650: f64, t2409: f64, t26880: f64, t3959: f64, t13893: f64, t4150: f64, t4002: f64, t8669: f64, t8743: f64, t13808: f64, t14596: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54717 = 7.0_f64 / 1152.0_f64 * t54716;
    let t54719 = t51650 * t4130;
    let t54722 = t3959 * t2409 * t26880;
    let t54724 = t13893 * t4150;
    let t54727 = 7.0_f64 / 144.0_f64 * t8669 * t4002;
    let t54729 = 7.0_f64 / 144.0_f64 * t8743 * t4002;
    let t54730 = t13808 * t14596;
    (t54717, t54719, t54722, t54724, t54727, t54729, t54730)
}
