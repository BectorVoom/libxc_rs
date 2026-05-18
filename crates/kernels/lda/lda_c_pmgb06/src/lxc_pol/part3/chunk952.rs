//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 952/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk952<F: Float>(t11155: F, t2151: F, t3734: F, t11140: F, t11141: F, t11142: F, t11145: F, t11147: F, t11150: F, t11152: F, t8798: F, t8814: F, t8822: F, t8824: F, t8826: F, t8830: F, t8834: F) -> F {
    let t11156 = F::new(0.0007324578922402618) * t11155;
    let t11157 = t2151 * t3734;
    let t11159 = t11140 - t8798 - t11141 + F::new(3.5089341735807875) * t11142 - F::new(0.0005493434191801964) * t11145 - F::new(51.94757731704439) * t11147 - t11150 - t11152 + t8814 + t8822 - F::new(1.7544670867903938) * t8824 - F::new(51.94757731704439) * t8826 + t8830 - t8834 + t11156 - F::new(0.0005696894717424259) * t11157;
    t11159
}
