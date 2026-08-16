//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 710/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk710(t4993: f64, t514: f64, t454: f64, t6950: f64, t1832: f64, t6292: f64, t1828: f64, t6488: f64, t1823: f64, t6477: f64, t4977: f64, t1948: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6951 = t514 * t4993;
    let t6952 = t454 * t6951;
    let t6954 = 0.08230132705969918_f64 * t6950 * t6952;
    let t6956 = 3.7610742193750633_f64 * t1832 * t6292;
    let t6958 = 1.8805371096875316_f64 * t1828 * t6292;
    let t6962 = 2.507382812916709_f64 * t1832 * t6488;
    let t6964 = t1823 * t6477;
    let t6966 = t514 * t4977;
    let t6967 = t454 * t6966;
    let t6969 = 0.04115066352984959_f64 * t1948 * t6967;
    (t6954, t6956, t6958, t6962, t6964, t6969)
}
