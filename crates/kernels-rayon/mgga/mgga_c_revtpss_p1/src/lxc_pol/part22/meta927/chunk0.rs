//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3151/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3151(t17728: f64, t3555: f64, t489: f64, t12772: f64, t17736: f64, t17738: f64, t3623: f64, t53739: f64, t13127: f64, t12865: f64, t3746: f64, t12831: f64, t17395: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56861 = t3555 * t489 * t17728;
    let t56867 = t17736 * t12772 * t17738;
    let t56878 = t3623 * t53739;
    let t56879 = t13127 * t56878;
    let t56888 = t3746 * t12865;
    let t56953 = t12831 * t17395;
    (t56861, t56867, t56878, t56879, t56888, t56953)
}
