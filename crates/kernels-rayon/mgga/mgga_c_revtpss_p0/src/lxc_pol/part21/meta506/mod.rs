//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta506(t15957: f64, t3095: f64, t3092: f64, t2857: f64, t357: f64, t2251: f64, t4781: f64, t11659: f64, t3154: f64, t1592: f64, t11710: f64, t4782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15958, t15959, t15963, t15964, t15965, t15968, t15969, t15970, t15973, t15974, t15975, t15984) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2125(t15957, t3095, t3092, t2857, t357, t2251, t4781, t11659, t3154, t1592, t11710, t4782);
    (t15958, t15959, t15963, t15964, t15965, t15968, t15969, t15970, t15973, t15974, t15975, t15984)
}
