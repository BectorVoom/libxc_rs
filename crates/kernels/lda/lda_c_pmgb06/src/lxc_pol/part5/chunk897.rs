//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 897/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk897<F: Float>(t1179: F, t186: F, t543: F, t55: F, t27: F, t3024: F, t545: F, t188: F, t3023: F, t398: F, t4641: F, t4913: F, t83: F) -> (F, F, F, F) {
    let t10711 = F::new(0.2244364134416412) * t543 * t55 * t1179 * t186;
    let t10714 = F::new(0.4328416544945937) * t3024 * t27 * t545;
    let t10720 = t398 * t3023 * t188;
    let t10727 = F::new(4.0) / F::new(3.0) * t83 * (-F::new(4.277777777777778) * t4641 + F::new(220.0) / F::new(81.0) * t4913) * t188;
    (t10711, t10714, t10720, t10727)
}
