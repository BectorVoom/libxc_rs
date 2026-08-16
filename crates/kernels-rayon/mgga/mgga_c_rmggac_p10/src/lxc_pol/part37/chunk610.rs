//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 610/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk610(t15456: f64, t1971: f64, t3351: f64, t15197: f64, t2347: f64, t699: f64, t1550: f64, t2350: f64, t903: f64, t2211: f64, t2392: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15457 = t1971 * t15456;
    let t15458 = t3351 * t15457;
    let t15459 = 0.12769379967989351819e-4_f64 * t15458;
    let t15460 = 0.10227998120342003148e-1_f64 * t15197;
    let t15464 = t699 * t2347;
    let t15465 = t1550 * t15464;
    let t15466 = 0.2993560425465952141e-1_f64 * t15465;
    let t15467 = t699 * t2350;
    let t15468 = t903 * t15467;
    let t15469 = 0.44903406381989282115e-1_f64 * t15468;
    let t15470 = t2211 * t2392;
    let t15471 = t739 * t15470;
    (t15457, t15459, t15460, t15464, t15466, t15467, t15469, t15470, t15471)
}
