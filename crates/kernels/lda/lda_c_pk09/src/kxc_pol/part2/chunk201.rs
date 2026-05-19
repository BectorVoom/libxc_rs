//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 201/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk201<F: Float>(t612: F, t616: F, t626: F, t636: F, t653: F, t667: F, t671: F, t676: F, t681: F, t687: F) -> (F, F, F) {
    let t689 = F::cast_from(0.505765839233979_f64) * t612;
    let t690 = F::cast_from(0.337177226155986_f64) * t616;
    let t694 = t667 + t671 + F::new(6.0) * t676 + F::new(6.0) * t681 - F::new(6.0) * t687 + t689 + t690 + F::cast_from(0.505765839233979_f64) * t626 + F::cast_from(0.505765839233979_f64) * t636 - F::cast_from(0.505765839233979_f64) * t653;
    (t689, t690, t694)
}
