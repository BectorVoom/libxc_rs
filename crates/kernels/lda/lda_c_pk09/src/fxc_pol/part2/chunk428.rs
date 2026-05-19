//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 428/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk428<F: Float>(t2159: F, t2163: F, t2167: F, t2171: F, t2175: F, t2179: F, t667: F, t671: F, t689: F, t690: F) -> F {
    let t2192 = t667 + t671 + F::new(6.0) * t2159 + F::new(6.0) * t2163 - F::new(6.0) * t2167 + t689 + t690 + F::cast_from(0.505765839233979_f64) * t2171 + F::cast_from(0.505765839233979_f64) * t2175 - F::cast_from(0.505765839233979_f64) * t2179;
    t2192
}
