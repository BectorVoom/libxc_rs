//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 986/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk986<F: Float>(t1135: F, t868: F, t11720: F, t11723: F, t11726: F, t11729: F, t11731: F, t4209: F, t795: F, t9045: F, t9048: F, t9050: F, t9052: F, t9061: F) -> F {
    let t11733 = t1135 * t868;
    let t11740 = -F::new(0.0837628205355044) * t795 * t4209 + F::new(0.2512884616065132) * t11720 - F::new(1.7083556008645087) * t11723 + F::new(0.19455129084526285) * t11726 + F::new(0.05969187332752383) * t11729 + F::new(0.5025769232130264) * t11731 + F::new(0.5025769232130264) * t11733 - F::new(0.5025769232130264) * t9045 + F::new(0.0837628205355044) * t9048 + F::new(0.2512884616065132) * t9050 + F::new(0.2512884616065132) * t9052 - F::new(0.5025769232130264) * t9061;
    t11740
}
