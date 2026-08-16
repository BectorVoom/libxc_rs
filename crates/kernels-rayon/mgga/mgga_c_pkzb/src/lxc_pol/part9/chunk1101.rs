//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1101/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1101(t53: f64, t5633: f64, t179: f64, t299: f64, t5635: f64, t2002: f64, t220: f64, t5629: f64, t771: f64, t5680: f64, t2068: f64, t5537: f64) -> (f64, f64, f64, f64, f64) {
    let t18204 = t53 * t5633;
    let t18207 = t299 * t179 * t18204 * t5635;
    let t18210 = 1.0_f64 / t2002 / t220;
    let t18216 = t771 * t5629;
    let t18218 = t771 * t5680;
    let t18232 = t299 * t179 * t2068 * t5537;
    (t18207, t18210, t18216, t18218, t18232)
}
