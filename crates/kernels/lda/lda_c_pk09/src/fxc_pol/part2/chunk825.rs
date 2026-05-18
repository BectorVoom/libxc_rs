//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 825/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk825<F: Float>(t1125: F, t3744: F, t3758: F, t3775: F, t3778: F, t3821: F, t3844: F, t8315: F, t8318: F, t8322: F, t8326: F, t8331: F, t8334: F, t8338: F, t8342: F, t8345: F) -> F {
    let t8347 = -F::new(2.427516195194328) * t3775 + F::new(2.427516195194328) * t3778 - F::new(3.7610742193750633) * t3821 - F::new(1.8805371096875316) * t3844 - t1125 * t8315 - F::new(4.738783832122567) * t3758 * t8318 + F::new(4.738783832122567) * t3758 * t8322 + F::new(1.1846959580306418) * t3744 * t8326 - F::new(4.738783832122567) * t8331 - F::new(4.738783832122567) * t3758 * t8334 - F::new(7.108175748183851) * t3758 * t8338 + F::new(4.738783832122567) * t3758 * t8342 - F::new(1.2536914064583544) * t8345;
    t8347
}
