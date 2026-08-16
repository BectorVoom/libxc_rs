//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 838/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk838(t4899: f64, t4903: f64, t4905: f64, t4909: f64, t4915: f64, t4917: f64, t4919: f64, t4921: f64, t4923: f64, t4925: f64, t4927: f64, t4932: f64, t4935: f64, t4940: f64, t4945: f64, t4948: f64, t4950: f64) -> f64 {
    let t5857 = -t4899 + t4903 + t4905 - t4909 - t4915 + t4917 + t4919 - t4921 - t4923 - t4925 - t4927 - t4932 - t4935 - t4940 + t4945 - t4948 - t4950;
    t5857
}
