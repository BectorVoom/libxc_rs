//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1285/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1285(t11834: f64, t11848: f64, t15848: f64, t15850: f64, t15887: f64, t22722: f64, t22725: f64, t22728: f64, t22731: f64, t22734: f64, t22737: f64, t22740: f64, t22743: f64, t22747: f64, t22786: f64, t22788: f64, t22795: f64, t22798: f64, t22801: f64, t22804: f64) -> f64 {
    let t23025 = 0.003778333333333333_f64 * t15848 - 0.0012594444444444445_f64 * t15850 - 0.005877407407407408_f64 * t11834 + 0.005037777777777778_f64 * t11848 + 0.0016792592592592592_f64 * t15887 + 0.003778333333333333_f64 * t22722 + 0.007556666666666666_f64 * t22725 - 0.04534_f64 * t22728 + 0.06801_f64 * t22731 - 0.011335_f64 * t22734 - 0.02267_f64 * t22737 + 0.04534_f64 * t22740 - 0.02518888888888889_f64 * t22743 - 0.04534_f64 * t22747 + 0.003778333333333333_f64 * t22786 - 0.0012594444444444445_f64 * t22788 - 0.003778333333333333_f64 * t22795 + 0.0012594444444444445_f64 * t22798 + 0.005597530864197531_f64 * t22801 + 0.012594444444444445_f64 * t22804;
    t23025
}
