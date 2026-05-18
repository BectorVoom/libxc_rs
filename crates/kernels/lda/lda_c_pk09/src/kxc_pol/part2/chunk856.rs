//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 856/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk856<F: Float>(t609: F, t8732: F, t891: F, t4587: F, t623: F, t896: F, t633: F, t903: F, t143: F, t151: F, t2206: F, t3744: F, t4475: F, t8096: F, t8101: F, t8760: F, t8816: F, t8821: F, t8829: F, t933: F, t98: F) -> F {
    let t8836 = t891 * t8732 * t609;
    let t8837 = t4587 * t8836;
    let t8840 = t896 * t8732 * t623;
    let t8844 = t903 * t8732 * t633;
    let t8848 = F::new(3.7610742193750633) * t8760 + F::new(1.8805371096875316) * t8816 * t98 + F::new(22.07984838129906) * t8821 + F::new(3.7610742193750633) * t143 * t8096 + F::new(3.7610742193750633) * t143 * t8101 - F::new(0.6268457032291772) * t933 * t2206 - F::new(1.8805371096875316) * t8829 - F::new(1.8805371096875316) * t151 * t8096 - F::new(1.8805371096875316) * t151 * t8101 + F::new(1.1846959580306418) * t8837 + F::new(1.1846959580306418) * t3744 * t8840 + F::new(1.1846959580306418) * t3744 * t8844 + F::new(22.07984838129906) * t4475;
    t8848
}
