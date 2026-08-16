//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 837/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk837(t17021: f64, t379: f64, t9133: f64, t12969: f64, t3478: f64, t12968: f64, t1017: f64, t2178: f64, t3483: f64, t13140: f64, t13153: f64, t3425: f64) -> (f64, f64, f64, f64) {
    let t17022 = t17021 * t379;
    let t17023 = t9133 * t17022;
    let t17026 = t12969 * t3478;
    let t17027 = t12968 * t17026;
    let t17030 = t2178 * t1017;
    let t17031 = t17030 * t3483;
    let t17032 = t13140 * t17031;
    let t17035 = t13153 * t3425;
    (t17023, t17027, t17032, t17035)
}
