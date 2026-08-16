//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1010/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1010(t2329: f64, t72109: f64, t2344: f64, t71229: f64, t14581: f64, t8526: f64, t75758: f64, t71630: f64, t75789: f64, t71634: f64, t15470: f64, t2604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78274 = t72109 * t2329;
    let t78275 = 0.13637330827122670864e-1_f64 * t78274;
    let t78276 = t71229 * t2344;
    let t78277 = 0.10227998120342003148e-1_f64 * t78276;
    let t78278 = t14581 * t8526;
    let t78279 = 0.10227998120342003148e-1_f64 * t78278;
    let t78280 = 0.14967802127329760705e-1_f64 * t75758;
    let t78287 = 0.18183107769496894486e-1_f64 * t71630;
    let t78288 = 0.19709219354514038085e-5_f64 * t75789;
    let t78290 = 0.99317399751028291929e-5_f64 * t71634;
    let t78294 = t2604 * t15470;
    (t78275, t78277, t78279, t78280, t78287, t78288, t78290, t78294)
}
