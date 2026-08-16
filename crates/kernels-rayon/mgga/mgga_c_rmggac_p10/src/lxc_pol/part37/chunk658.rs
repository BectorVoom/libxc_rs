//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 658/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk658(t38350: f64, t674: f64, t5542: f64, t8607: f64, t8687: f64, t3924: f64, t623: f64, t34760: f64, t8450: f64, t2185: f64, t2338: f64, t7556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38351 = t38350 * t674;
    let t38354 = t8607 * t5542;
    let t38355 = t38354 * t674;
    let t38471 = t8687 * t5542;
    let t38472 = t38471 * t674;
    let t38495 = t623 * t3924;
    let t38530 = t8450 * t34760;
    let t38638 = t8450 * t2185;
    let t38839 = t2338 * t7556;
    (t38351, t38354, t38355, t38471, t38472, t38495, t38530, t38638, t38839)
}
