//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 618/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk618(t1447: f64, t1916: f64, t1920: f64, t1730: f64, t871: f64, t3213: f64, t806: f64, t1872: f64, t441: f64, t1504: f64, t831: f64, t1848: f64, t490: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4723 = 8.0_f64 / 135.0_f64 * t1447 * t1916;
    let t4725 = 4.0_f64 / 81.0_f64 * t1447 * t1920;
    let t4740 = t871 * t1730;
    let t4777 = t3213 * t806;
    let t4779 = t441 * t1872;
    let t4786 = 2.0_f64 / 45.0_f64 * t831 * t1504;
    let t4788 = 2.0_f64 / 45.0_f64 * t1848 * t490;
    (t4723, t4725, t4740, t4777, t4779, t4786, t4788)
}
