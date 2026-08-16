//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 428/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk428(t2159: f64, t2163: f64, t2167: f64, t2171: f64, t2175: f64, t2179: f64, t667: f64, t671: f64, t689: f64, t690: f64) -> f64 {
    let t2192 = t667 + t671 + 6.0_f64 * t2159 + 6.0_f64 * t2163 - 6.0_f64 * t2167 + t689 + t690 + 0.505765839233979_f64 * t2171 + 0.505765839233979_f64 * t2175 - 0.505765839233979_f64 * t2179;
    t2192
}
