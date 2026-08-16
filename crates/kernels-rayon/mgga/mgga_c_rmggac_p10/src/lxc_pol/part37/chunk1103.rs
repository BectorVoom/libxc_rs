//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1103/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1103(t3281: f64, t558: f64, t352: f64, t5266: f64, t69181: f64, t76212: f64, t76216: f64, t76218: f64, t76222: f64, t76224: f64, t77907: f64, t77908: f64, t77911: f64, t77917: f64, t77920: f64, t77921: f64) -> (f64, f64) {
    let t80429 = t3281 * t558;
    let t80433 = t77907 + t77908 - t77911 + t77917 - t77920 + t77921 + t76212 - t76216 - t76218 + 0.11974241701863808564e0_f64 * t5266 * t80429 * t352 - t76222 - t76224 - t69181;
    (t80429, t80433)
}
