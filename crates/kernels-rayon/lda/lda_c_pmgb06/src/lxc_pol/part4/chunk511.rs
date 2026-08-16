//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 511/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk511(t1424: f64, t1448: f64, t1505: f64, t1518: f64, t1991: f64, t1997: f64, t1999: f64, t2001: f64, t2004: f64, t2006: f64, t2009: f64, t2014: f64, t2017: f64, t2020: f64, t2022: f64, t2027: f64, t2030: f64, t2032: f64, t213: f64) -> (f64, f64, f64, f64, f64) {
    let t2034 = 2.0_f64 / 135.0_f64 * t1424;
    let t2035 = 2.0_f64 / 135.0_f64 * t1448;
    let t2036 = t1505 / 45.0_f64;
    let t2037 = t1518 / 45.0_f64;
    let t2038 = t1991 + t1997 + t1999 + t2001 + t2004 + t2006 + t2009 + t2014 - t2017 - t2020 + t2022 * t213 / 3.0_f64 + t2027 / 3.0_f64 + 0.06077777777777778_f64 * t2030 + 2.0_f64 / 9.0_f64 * t2032 + t2034 + t2035 + t2036 + t2037;
    (t2034, t2035, t2036, t2037, t2038)
}
