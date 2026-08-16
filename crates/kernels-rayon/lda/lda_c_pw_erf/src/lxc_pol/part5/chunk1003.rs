//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1003/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1003(t325: f64, t6501: f64, t4606: f64, t6535: f64, t6538: f64, t6541: f64, t1251: f64, t5992: f64, t1245: f64, t2430: f64, t925: f64, t518: f64, t6874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15836 = t325 * t6501;
    let t15838 = t4606 * t6535;
    let t15848 = t325 * t6538;
    let t15850 = t325 * t6541;
    let t15852 = t1251 * t5992;
    let t15867 = t1245 * t5992;
    let t15887 = t925 * t2430;
    let t15926 = t6874 * t518;
    (t15836, t15838, t15848, t15850, t15852, t15867, t15887, t15926)
}
