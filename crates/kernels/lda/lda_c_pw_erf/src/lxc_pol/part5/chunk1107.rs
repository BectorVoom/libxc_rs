//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1107/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1107<F: Float>(t11834: F, t11848: F, t15848: F, t15850: F, t15887: F, t22722: F, t22725: F, t22728: F, t22731: F, t22734: F, t22737: F, t22740: F, t22743: F, t22747: F, t22786: F, t22788: F, t22795: F, t22798: F, t22801: F, t22804: F) -> (F,) {
    let t23025 = 0.003778333333333333 * t15848 - 0.0012594444444444445 * t15850 - 0.005877407407407408 * t11834 + 0.005037777777777778 * t11848 + 0.0016792592592592592 * t15887 + 0.003778333333333333 * t22722 + 0.007556666666666666 * t22725 - 0.04534 * t22728 + 0.06801 * t22731 - 0.011335 * t22734 - 0.02267 * t22737 + 0.04534 * t22740 - 0.02518888888888889 * t22743 - 0.04534 * t22747 + 0.003778333333333333 * t22786 - 0.0012594444444444445 * t22788 - 0.003778333333333333 * t22795 + 0.0012594444444444445 * t22798 + 0.005597530864197531 * t22801 + 0.012594444444444445 * t22804;
    (t23025,)
}
