//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1285/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1285<F: Float>(t11834: F, t11848: F, t15848: F, t15850: F, t15887: F, t22722: F, t22725: F, t22728: F, t22731: F, t22734: F, t22737: F, t22740: F, t22743: F, t22747: F, t22786: F, t22788: F, t22795: F, t22798: F, t22801: F, t22804: F) -> F {
    let t23025 = F::cast_from(0.003778333333333333_f64) * t15848 - F::cast_from(0.0012594444444444445_f64) * t15850 - F::cast_from(0.005877407407407408_f64) * t11834 + F::cast_from(0.005037777777777778_f64) * t11848 + F::cast_from(0.0016792592592592592_f64) * t15887 + F::cast_from(0.003778333333333333_f64) * t22722 + F::cast_from(0.007556666666666666_f64) * t22725 - F::cast_from(0.04534_f64) * t22728 + F::cast_from(0.06801_f64) * t22731 - F::cast_from(0.011335_f64) * t22734 - F::cast_from(0.02267_f64) * t22737 + F::cast_from(0.04534_f64) * t22740 - F::cast_from(0.02518888888888889_f64) * t22743 - F::cast_from(0.04534_f64) * t22747 + F::cast_from(0.003778333333333333_f64) * t22786 - F::cast_from(0.0012594444444444445_f64) * t22788 - F::cast_from(0.003778333333333333_f64) * t22795 + F::cast_from(0.0012594444444444445_f64) * t22798 + F::cast_from(0.005597530864197531_f64) * t22801 + F::cast_from(0.012594444444444445_f64) * t22804;
    t23025
}
