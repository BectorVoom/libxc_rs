//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 821/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk821(t419: f64, t421: f64, t5617: f64, t1186: f64, t2329: f64, t1193: f64, t1354: f64, t4429: f64, t118: f64, t2174: f64, t4622: f64, t4624: f64, t4626: f64, t4628: f64, t4630: f64, t4684: f64, t4721: f64, t4723: f64, t4725: f64, t4727: f64, t4730: f64, t4734: f64, t4735: f64, t4738: f64, t4739: f64) -> (f64, f64, f64, f64, f64) {
    let t5620 = 0.003950778065781896_f64 * t5617 * t419 * t421;
    let t5622 = t2329 * t1186 * t421;
    let t5625 = t4429 * t1193 * t1354;
    let t5627 = t2174 * t118;
    let t5629 = t4622 + t4624 + t4626 + t4628 + t4630 + t4684 - t4721 - t4723 + t4725 - t4727 - t4730 - t4734 - t4735 - t4738 - t4739;
    (t5620, t5622, t5625, t5627, t5629)
}
