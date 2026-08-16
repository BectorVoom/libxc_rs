//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 940/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk940(t32792: f64, t6547: f64, t23204: f64, t32866: f64, t6562: f64, t32809: f64, t8335: f64, t86893: f64, t214: f64, t7510: f64, t32867: f64, t32875: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t118858 = t6547 * t32792;
    let t118885 = t6562 * t23204 * t32866;
    let t118893 = t6547 * t32809;
    let t118903 = t6562 * t86893 * t8335;
    let t118910 = t214 * t7510;
    let t118915 = t6547 * t32867;
    let t118927 = t6547 * t32875;
    (t118858, t118885, t118893, t118903, t118910, t118915, t118927)
}
