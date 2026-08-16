//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1034/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1034(t4669: f64, t558: f64, t71903: f64, t71949: f64, t71940: f64, t326: f64, t650: f64, t9565: f64, t333: f64, t352: f64, t5155: f64, t5266: f64, t69181: f64, t69183: f64, t76212: f64, t76216: f64, t76218: f64, t76222: f64, t76224: f64, t77890: f64) -> f64 {
    let t77916 = t4669 * t71903 * t558;
    let t77917 = 0.44903406381989282115e-1_f64 * t77916;
    let t77919 = t4669 * t71949 * t558;
    let t77920 = 0.11974241701863808564e0_f64 * t77919;
    let t77921 = 0.39914139006212695213e-1_f64 * t71940;
    let t77929 = t326 * t9565 * t650;
    let t77930 = 0.34093327067806677161e-2_f64 * t77929;
    let t77931 = t77917 - t77920 + t77921 + 0.23948483403727617128e0_f64 * t5155 * t77890 * t333 + 0.11974241701863808564e0_f64 * t5266 * t77890 * t352 + t76212 - t76216 - t76218 - t76222 - t76224 - t69181 - t69183 + t77930;
    t77931
}
