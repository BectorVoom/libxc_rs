//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2335/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2335(t75795: f64, t7769: f64, t26135: f64, t5371: f64, t112: f64, t28868: f64, t16524: f64, t26550: f64, t55353: f64, t16521: f64, t7467: f64, t1873: f64, t19534: f64, t3941: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100902 = 54.0_f64 * t75795 * t7769;
    let t100908 = 27.0_f64 * t5371 * t26135;
    let t100911 = t28868 * t112;
    let t100915 = 54.0_f64 * t16524 * t26550;
    let t100917 = 54.0_f64 * t55353 * t7769;
    let t100921 = 27.0_f64 * t16521 * t7467;
    let t100924 = 27.0_f64 * t3941 * t1873 * t19534;
    (t100902, t100908, t100911, t100915, t100917, t100921, t100924)
}
