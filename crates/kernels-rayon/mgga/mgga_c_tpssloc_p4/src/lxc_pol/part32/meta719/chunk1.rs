//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2286/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2286(t3941: f64, t4072: f64, t7467: f64, t28017: f64, t3938: f64, t12524: f64, t28899: f64, t75795: f64, t7769: f64, t26135: f64, t5371: f64, t16524: f64, t26550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100893 = 54.0_f64 * t3941 * t7467 * t4072;
    let t100897 = 0.135e2_f64 * t3938 * t28017;
    let t100899 = 27.0_f64 * t12524 * t28899;
    let t100902 = 54.0_f64 * t75795 * t7769;
    let t100908 = 27.0_f64 * t5371 * t26135;
    let t100915 = 54.0_f64 * t16524 * t26550;
    (t100893, t100897, t100899, t100902, t100908, t100915)
}
