//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 900/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk900(t229: f64, t268: f64, t6559: f64, t225: f64, t23228: f64, t2056: f64, t40772: f64, t111: f64, t7758: f64, t25: f64, t1408: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81651 = t6559 * t229 * t268;
    let t82074 = t23228 * t225;
    let t84766 = t2056 * t40772;
    let t86647 = t7758 * t111;
    let t86716 = t40772 * t25;
    let t86721 = t2752 * t1408;
    (t81651, t82074, t84766, t86647, t86716, t86721)
}
