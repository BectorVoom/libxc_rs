//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1167/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1167(t16901: f64, t501: f64, t7024: f64, t16910: f64, t16917: f64, t16919: f64, t16929: f64, t2609: f64, t5152: f64, t114: f64, t557: f64, t6798: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20346 = 0.15584273195113317383e3_f64 * t16901;
    let t20347 = t501 * t7024;
    let t20348 = 12.0_f64 * t20347;
    let t20349 = 8.0_f64 * t16910;
    let t20350 = 0.18311447306006545054e-3_f64 * t16917;
    let t20351 = 0.73245789224026180215e-3_f64 * t16919;
    let t20352 = 960.0_f64 * t16929;
    let t20353 = t2609 * t5152;
    let t20354 = 0.10254018858216406658e4_f64 * t20353;
    let t20356 = t6798 * t114 * t557;
    (t20346, t20348, t20349, t20350, t20351, t20352, t20354, t20356)
}
