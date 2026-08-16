//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2994/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2994(t15758: f64, t16055: f64, t1063: f64, t15833: f64, t3172: f64, t11779: f64, t4845: f64, t15749: f64, t3211: f64, t16148: f64, t4837: f64, t1042: f64, t11656: f64, t15791: f64, t15906: f64, t15908: f64, t15970: f64, t16052: f64, t16138: f64, t16144: f64, t2858: f64, t3117: f64, t3127: f64, t3188: f64, t4801: f64, t53459: f64, t54479: f64) -> f64 {
    let t54623 = t15758 * t16055;
    let t54638 = t1063 * t3172 * t15833;
    let t54646 = t11779 * t4845;
    let t54648 = t3211 * t15749;
    let t54651 = t4837 * t3172 * t16148;
    let t54653 = 0.17149607247227894789e-2_f64 * t54623 - 0.45732285992607719436e-2_f64 * t16052 * t15970 - 0.38586616306262763275e-2_f64 * t15906 * t3117 * t54479 * t15908 + 0.85748036236139473944e-3_f64 * t3127 * t1042 * t16138 * t2858 - 0.45732285992607719436e-2_f64 * t11656 * t16144 + 0.95275595817932748827e-3_f64 * t54638 - 0.17149607247227894789e-2_f64 * t3188 * t15791 - 0.85748036236139473944e-3_f64 * t1063 * t1042 * t4801 * t53459 - 0.14481890564325777821e-1_f64 * t54646 - 0.7622047665434619906e-3_f64 * t54648 + 0.57165357490759649295e-3_f64 * t54651;
    t54653
}
