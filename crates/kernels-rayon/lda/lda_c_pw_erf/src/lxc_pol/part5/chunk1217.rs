//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1217/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1217(t3675: f64, t7520: f64, t1325: f64, t1440: f64, t494: f64, t519: f64, t542: f64, t9223: f64, t2098: f64, t6903: f64, t34: f64, t4956: f64, t6997: f64) -> (f64, f64, f64, f64) {
    let t21945 = t3675 * t7520;
    let t21949 = 8.0_f64 / 5.0_f64 * t1325 * t1440 * t21945 * t494;
    let t21954 = 16.0_f64 / 5.0_f64 * t519 * t1440 * t9223 * t7520 * t542;
    let t21958 = 12.0_f64 / 5.0_f64 * t519 * t1440 * t6903 * t2098;
    let t21962 = 4.0_f64 / 5.0_f64 * t1325 * t4956 * t6997 * t34;
    (t21949, t21954, t21958, t21962)
}
