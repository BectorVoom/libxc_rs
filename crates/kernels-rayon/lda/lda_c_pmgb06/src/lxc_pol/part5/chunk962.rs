//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 962/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk962(t1908: f64, t5220: f64, t1447: f64, t6518: f64, t1423: f64, t6524: f64, t6783: f64, t1925: f64, t5194: f64, t6788: f64, t5105: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15196 = t5220 * t1908;
    let t15208 = t1447 * t6518;
    let t15216 = t1423 * t6524;
    let t15237 = t1447 * t6783;
    let t15244 = t5194 * t1925;
    let t15248 = t1423 * t6788;
    let t15256 = t831 * t5105;
    (t15196, t15208, t15216, t15237, t15244, t15248, t15256)
}
