//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1276/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1276(t61062: f64, t764: f64, t238: f64, t5543: f64, t1695: f64, t212: f64, t60720: f64, t2376: f64, t339: f64, t5557: f64, t803: f64, t228: f64, t32386: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61063 = t61062 * t764;
    let t61072 = t5543 * t238;
    let t61079 = t60720 * t212 * t1695;
    let t61086 = t339 * t5557 * t2376;
    let t61087 = t61086 * t803;
    let t61195 = t32386 * t228;
    (t61063, t61072, t61079, t61086, t61087, t61195)
}
