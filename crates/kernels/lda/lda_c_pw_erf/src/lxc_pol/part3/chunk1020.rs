//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1020/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1020<F: Float>(t11948: F, t1511: F, t184: F, t1980: F, t199: F, t1529: F, t1960: F, t9267: F, t9270: F, t9273: F, t11937: F, t11939: F, t11941: F, t11943: F, t11945: F, t11947: F, t9259: F) -> (F, F, F, F, F, F, F) {
    let t11949 = F::new(4.0) / F::new(3.0) * t11948;
    let t11953 = F::new(4.0) / F::new(5.0) * t1511 * t1980 * t184 * t199;
    let t11954 = t1960 * t1529;
    let t11955 = F::new(4.0) / F::new(45.0) * t11954;
    let t11956 = F::new(16.0) / F::new(45.0) * t9267;
    let t11957 = F::new(8.0) / F::new(45.0) * t9270;
    let t11958 = F::new(16.0) / F::new(45.0) * t9273;
    let t11959 = F::new(4.0) / F::new(3.0) * t9259 + t11937 - t11939 + t11941 - t11943 - t11945 - t11947 - t11949 + t11953 - t11955 + t11956 - t11957 - t11958;
    (t11949, t11953, t11955, t11956, t11957, t11958, t11959)
}
