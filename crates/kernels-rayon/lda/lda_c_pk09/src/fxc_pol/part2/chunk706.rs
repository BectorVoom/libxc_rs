//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 706/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk706(t1948: f64, t6822: f64, t142: f64, t6586: f64, t551: f64, t6517: f64, t1665: f64, t1972: f64, t1672: f64, t1982: f64, t6319: f64, t6325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6823 = t1948 * t6822;
    let t6825 = t6586 * t142;
    let t6827 = t6825 * t551 * t6517;
    let t6829 = t1972 * t1665;
    let t6831 = t1982 * t1672;
    let t6836 = 0.9421211958699838_f64 * t6319;
    let t6838 = 0.6280807972466558_f64 * t6325;
    (t6823, t6825, t6827, t6829, t6831, t6836, t6838)
}
