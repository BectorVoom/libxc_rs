//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 201/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk201(t612: f64, t616: f64, t626: f64, t636: f64, t653: f64, t667: f64, t671: f64, t676: f64, t681: f64, t687: f64) -> (f64, f64, f64) {
    let t689 = 0.505765839233979_f64 * t612;
    let t690 = 0.337177226155986_f64 * t616;
    let t694 = t667 + t671 + 6.0_f64 * t676 + 6.0_f64 * t681 - 6.0_f64 * t687 + t689 + t690 + 0.505765839233979_f64 * t626 + 0.505765839233979_f64 * t636 - 0.505765839233979_f64 * t653;
    (t689, t690, t694)
}
