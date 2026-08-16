//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1215/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1215(t11019: f64, t1137: f64, t154: f64, t17874: f64, t17881: f64, t17897: f64, t17902: f64, t2031: f64, t2104: f64, t2105: f64, t2106: f64, t21500: f64, t21538: f64, t21543: f64, t25221: f64, t25248: f64, t25275: f64, t25290: f64, t25337: f64, t25351: f64, t25357: f64, t26387: f64, t276: f64, t287: f64, t2899: f64, t2900: f64, t2922: f64, t29813: f64, t3542: f64, t3645: f64, t655: f64, t742: f64, t761: f64, t7664: f64, t7701: f64, t9161: f64, t9187: f64, t9258: f64, t9287: f64, t9292: f64) -> f64 {
    let t29858 = -t276 * t154 * t742 * t29813 / 96.0_f64 - 0.85748036236139473947e-3_f64 * t25248 - t21500 + 0.12862205435420921092e-2_f64 * t2922 * t25221 * t9292 + 0.25724410870841842183e-2_f64 * t25275 - 5.0_f64 / 432.0_f64 * t21538 - t21543 - 5.0_f64 / 486.0_f64 * t17874 - t17881 - t25290 / 96.0_f64 - 0.33875767401931644027e-3_f64 * t17897 - t17902 + 0.12862205435420921092e-2_f64 * t2922 * t25221 * t2900 * t2106 + 0.7717323261252552655e-2_f64 * t2899 * t9258 * t2031 * t9287 - 0.38586616306262763275e-2_f64 * t2922 * t9258 * t7701 * t11019 - 0.12862205435420921092e-2_f64 * t7664 * t25337 * t26387 * t3645 + 0.51448821741683684367e-2_f64 * t25351 - 0.1543464652250510531e-1_f64 * t2104 * t25357 * t761 * t3542 * t655 + 0.7717323261252552655e-2_f64 * t2104 * t9258 * t761 * t9187 - 0.12862205435420921092e-2_f64 * t2104 * t2105 * t1137 * t287 * t9161;
    t29858
}
