//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 935/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk935(t1692: f64, t2719: f64, t1020: f64, t1535: f64, t1634: f64, t2714: f64, t2718: f64, t5025: f64, t5028: f64, t5040: f64, t5066: f64, t5069: f64, t5073: f64, t5082: f64, t5186: f64, t5324: f64, t5333: f64, t5338: f64, t5344: f64, t7043: f64, t7045: f64, t7048: f64, t7049: f64, t7050: f64, t7051: f64) -> (f64, f64) {
    let t7209 = t2719 * t1692;
    let t7215 = -3.0_f64 * t1020 * t1535 * t5082 + 6.0_f64 * t1634 * t2714 * t2718 + 6.0_f64 * t2718 * t7209 + t5025 + t5028 + t5040 + t5066 - t5069 - t5073 + t5186 - t5324 + t5333 - t5338 - t5344 - t7043 + t7045 + t7048 + t7049 - t7050 - t7051;
    (t7209, t7215)
}
