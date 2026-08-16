//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 996/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk996(t1746: f64, t7314: f64, t1034: f64, t2343: f64, t40: f64, t344: f64, t6071: f64, t1064: f64, t2344: f64, t1067: f64, t6069: f64, t479: f64, t7032: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15421 = t7314 * t1746;
    let t15450 = t40 * t2343 * t1034;
    let t15453 = t344 * t6071;
    let t15455 = t1064 * t2344;
    let t15457 = t1067 * t2344;
    let t15461 = t344 * t6069;
    let t15481 = t7032 * t479;
    (t15421, t15450, t15453, t15455, t15457, t15461, t15481)
}
