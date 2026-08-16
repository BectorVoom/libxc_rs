//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2994/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2994<F: Float>(t15758: F, t16055: F, t1063: F, t15833: F, t3172: F, t11779: F, t4845: F, t15749: F, t3211: F, t16148: F, t4837: F, t1042: F, t11656: F, t15791: F, t15906: F, t15908: F, t15970: F, t16052: F, t16138: F, t16144: F, t2858: F, t3117: F, t3127: F, t3188: F, t4801: F, t53459: F, t54479: F) -> F {
    let t54623 = t15758 * t16055;
    let t54638 = t1063 * t3172 * t15833;
    let t54646 = t11779 * t4845;
    let t54648 = t3211 * t15749;
    let t54651 = t4837 * t3172 * t16148;
    let t54653 = F::cast_from(0.17149607247227894789e-2_f64) * t54623 - F::cast_from(0.45732285992607719436e-2_f64) * t16052 * t15970 - F::cast_from(0.38586616306262763275e-2_f64) * t15906 * t3117 * t54479 * t15908 + F::cast_from(0.85748036236139473944e-3_f64) * t3127 * t1042 * t16138 * t2858 - F::cast_from(0.45732285992607719436e-2_f64) * t11656 * t16144 + F::cast_from(0.95275595817932748827e-3_f64) * t54638 - F::cast_from(0.17149607247227894789e-2_f64) * t3188 * t15791 - F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t1042 * t4801 * t53459 - F::cast_from(0.14481890564325777821e-1_f64) * t54646 - F::cast_from(0.7622047665434619906e-3_f64) * t54648 + F::cast_from(0.57165357490759649295e-3_f64) * t54651;
    t54653
}
