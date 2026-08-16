//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1266/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1266(t1953: f64, t22285: f64, t503: f64, t11: f64, t21186: f64, t21190: f64, t1243: f64, t22281: f64, t21240: f64, t3536: f64, t11829: f64, t11834: f64, t11846: f64, t11848: f64, t15838: f64, t15848: f64, t15850: f64, t15887: f64, t17327: f64, t17332: f64, t2061: f64, t22713: f64, t22717: f64, t22722: f64, t22725: f64, t22728: f64, t25: f64, t3516: f64, t9761: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22731 = t1953 * t503 * t22285;
    let t22734 = t11 * t503 * t21186;
    let t22737 = t1953 * t503 * t21190;
    let t22740 = t11 * t1243 * t22281;
    let t22743 = t11 * t3536 * t21240;
    let t22745 = -0.2879333333333333_f64 * t15838 - 0.07198333333333333_f64 * t15848 + 0.023994444444444443_f64 * t15850 + 0.05925925925925926_f64 * t11829 + 0.11197407407407407_f64 * t11834 - 0.044444444444444446_f64 * t11846 - 0.007407407407407408_f64 * t17327 + 0.044444444444444446_f64 * t17332 - 0.09597777777777777_f64 * t11848 - 0.03199259259259259_f64 * t15887 - 0.006913580246913581_f64 * t25 * t9761 * t22713 - 0.017777777777777778_f64 * t2061 * t3516 * t22717 - 0.07198333333333333_f64 * t22722 - 0.14396666666666666_f64 * t22725 + 0.8638_f64 * t22728 - 1.2957_f64 * t22731 + 0.21595_f64 * t22734 + 0.4319_f64 * t22737 - 0.8638_f64 * t22740 + 0.47988888888888886_f64 * t22743;
    (t22731, t22734, t22737, t22740, t22743, t22745)
}
