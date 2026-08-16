//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1285/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1285(t12853: f64, t12855: f64, t12857: f64, t12860: f64, t12863: f64, t12870: f64, t12873: f64, t12876: f64, t12878: f64, t12880: f64, t12883: f64, t12885: f64, t12887: f64) -> f64 {
    let t15051 = -t12853 - t12855 - t12857 - t12860 + t12863 - t12870 - t12873 + t12876 + t12878 + t12880 + t12883 + t12885 - t12887;
    t15051
}
