//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 963/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk963(t118532: f64, t30716: f64, t112797: f64, t32844: f64, t13242: f64, t232: f64, t30714: f64, t4180: f64, t234: f64, t240: f64, t241: f64, t4248: f64, t776: f64, t812: f64, t9646: f64) -> (f64, f64, f64, f64) {
    let t118533 = t118532 * t30716;
    let t118535 = t112797 * t32844;
    let t118539 = t30714 * t4180 * t13242 * t232;
    let t118546 = t812 * t234 * t240 * t241 * t9646 * t4248 * t776;
    (t118533, t118535, t118539, t118546)
}
