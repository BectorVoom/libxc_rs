//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1996/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1996(t25374: f64, t93320: f64, t25410: f64, t93189: f64, t93174: f64, t93341: f64, t93169: f64, t93191: f64, t2439: f64, t7048: f64, t780: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93364 = t93320 * t25374;
    let t93371 = t93189 * t25410;
    let t93372 = t93371 * t93174;
    let t93374 = t93341 * t25410;
    let t93377 = t93169 * t25374;
    let t93378 = t93377 * t93191;
    let t93382 = t2439 * t785 * t7048 * t780;
    (t93364, t93371, t93372, t93374, t93377, t93378, t93382)
}
