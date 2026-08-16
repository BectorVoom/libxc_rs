//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1011/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1011(t78294: f64, t699: f64, t8700: f64, t903: f64, t75794: f64, t3225: f64, t39953: f64, t75800: f64, t75803: f64, t75811: f64, t75814: f64, t75818: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78295 = 0.2993560425465952141e-1_f64 * t78294;
    let t78297 = t903 * t699 * t8700;
    let t78298 = 0.44903406381989282115e-1_f64 * t78297;
    let t78299 = 0.79828278012425390427e-1_f64 * t75794;
    let t78300 = t39953 * t3225;
    let t78301 = 0.34093327067806677161e-2_f64 * t78300;
    let t78303 = 0.2627895913935205078e-5_f64 * t75800;
    let t78304 = 0.2627895913935205078e-5_f64 * t75803;
    let t78308 = 0.2627895913935205078e-5_f64 * t75811;
    let t78309 = 0.59127658063542114255e-5_f64 * t75814;
    let t78310 = 0.7661627980793611092e-4_f64 * t75818;
    (t78295, t78298, t78299, t78301, t78303, t78304, t78308, t78309, t78310)
}
