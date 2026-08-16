//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3018/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3018(t1041: f64, t1670: f64, t42994: f64, t11922: f64, t15786: f64, t4892: f64, t1042: f64, t1063: f64, t11779: f64, t11933: f64, t12160: f64, t15780: f64, t15787: f64, t16020: f64, t16040: f64, t16048: f64, t16052: f64, t16067: f64, t16068: f64, t16104: f64, t1665: f64, t3117: f64, t42340: f64, t43066: f64, t4806: f64, t4854: f64, t4899: f64, t4902: f64, t54450: f64, t54479: f64) -> f64 {
    let t55247 = t1041 * t42994 * t1670;
    let t55265 = t4892 * t11922 * t15786;
    let t55271 = 0.23818898954483187207e-3_f64 * t1063 * t1042 * t4806 * t54450 - 0.21722835846488666732e-1_f64 * t11779 * t4854 + 0.63517063878621832551e-4_f64 * t55247 + 0.68598428988911579154e-2_f64 * t12160 * t16048 * t4902 + 0.68598428988911579154e-2_f64 * t11933 * t16040 + 0.45732285992607719436e-2_f64 * t43066 * t16104 - 0.21722835846488666732e-1_f64 * t42340 * t1665 + 0.64311027177104605458e-3_f64 * t16067 * t3117 * t54479 * t16068 - 0.68598428988911579154e-2_f64 * t16052 * t15787 + 0.85748036236139473944e-3_f64 * t55265 - 0.64311027177104605458e-3_f64 * t4899 * t3117 * t15780 * t16020;
    t55271
}
