//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 762/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk762(t2143: f64, t56: f64, t92: f64, t2149: f64, t14: f64, t2988: f64, t2990: f64, t3007: f64, t3009: f64, t7704: f64, t7766: f64) -> f64 {
    let t7821 = t56 * t92 * t2143;
    let t7827 = t56 * t92 * t2149;
    let t7831 = -t2988 + t2990 / 3.0_f64 + t7821 / 3.0_f64 + t56 * t14 * t7766 - t3007 + t3009 / 3.0_f64 + t7827 / 3.0_f64 + t56 * t14 * t7704;
    t7831
}
