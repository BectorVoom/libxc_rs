//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1160/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1160(t1128: f64, t4794: f64, t1675: f64, t3356: f64, t1136: f64, t4820: f64, t1683: f64, t3351: f64, t3333: f64, t4823: f64, t1138: f64, t11410: f64, t11420: f64, t14864: f64, t14866: f64, t14916: f64, t14934: f64, t14939: f64, t3327: f64, t3332: f64, t3352: f64, t3360: f64, t4797: f64) -> f64 {
    let t15141 = t4794 * t1128;
    let t15146 = t1675 * t3356;
    let t15153 = t4820 * t1136;
    let t15156 = t1683 * t3351;
    let t15159 = t4823 * t3333;
    let t15162 = -0.19751673498613801407e-1_f64 * t14934 - t14864 - t14866 - t14916 + 2.0_f64 * t15141 * t1138 + 1.0_f64 * t4797 * t3352 + 0.32163958997385070134e2_f64 * t15146 * t3360 + 1.0_f64 * t11410 * t1683 + 2.0_f64 * t3327 * t4820 - t14939 - 4.0_f64 * t3332 * t15153 - 2.0_f64 * t3332 * t15156 - 0.19298375398431042081e3_f64 * t11420 * t15159;
    t15162
}
