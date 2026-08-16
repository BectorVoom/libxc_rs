//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2125/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2125(t15957: f64, t3095: f64, t3092: f64, t2857: f64, t357: f64, t2251: f64, t4781: f64, t11659: f64, t3154: f64, t1592: f64, t11710: f64, t4782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15958 = t15957 * t3095;
    let t15959 = t3092 * t15958;
    let t15962 = t357 * t2857;
    let t15963 = t15962 * t2251;
    let t15964 = t4781 * t15963;
    let t15965 = t3092 * t15964;
    let t15968 = t11659 * t3154;
    let t15969 = t1592 * t15968;
    let t15970 = t3092 * t15969;
    let t15973 = t11659 * t357;
    let t15974 = t1592 * t15973;
    let t15975 = t3092 * t15974;
    let t15984 = t11710 * t4782;
    (t15958, t15959, t15963, t15964, t15965, t15968, t15969, t15970, t15973, t15974, t15975, t15984)
}
