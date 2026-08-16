//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 397/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk397(t1240: f64, t514: f64, t454: f64, t1948: f64, t1672: f64, t498: f64, t502: f64, t490: f64, t1729: f64, t545: f64, t537: f64, t524: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1949 = t514 * t1240;
    let t1950 = t454 * t1949;
    let t1952 = 0.04115066352984959_f64 * t1948 * t1950;
    let t1954 = 0.6268457032291772_f64 * t498 * t1672;
    let t1956 = 6.496391258193384_f64 * t502 * t1672;
    let t1958 = 1.2536914064583544_f64 * t490 * t1672;
    let t1959 = t545 * t1729;
    let t1962 = t537 * t1729;
    let t1965 = t524 * t1729;
    (t1949, t1950, t1952, t1954, t1956, t1958, t1959, t1962, t1965)
}
