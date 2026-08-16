//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1064/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1064(t71630: f64, t75789: f64, t71634: f64, t15470: f64, t2604: f64, t699: f64, t8700: f64, t903: f64, t75794: f64, t3225: f64, t39953: f64, t75800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78287 = 0.18183107769496894486e-1_f64 * t71630;
    let t78288 = 0.19709219354514038085e-5_f64 * t75789;
    let t78290 = 0.99317399751028291929e-5_f64 * t71634;
    let t78294 = t2604 * t15470;
    let t78295 = 0.2993560425465952141e-1_f64 * t78294;
    let t78297 = t903 * t699 * t8700;
    let t78298 = 0.44903406381989282115e-1_f64 * t78297;
    let t78299 = 0.79828278012425390427e-1_f64 * t75794;
    let t78300 = t39953 * t3225;
    let t78301 = 0.34093327067806677161e-2_f64 * t78300;
    let t78303 = 0.2627895913935205078e-5_f64 * t75800;
    (t78287, t78288, t78290, t78295, t78298, t78299, t78301, t78303)
}
