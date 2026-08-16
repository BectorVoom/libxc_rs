//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1002/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1002(t12998: f64, t12974: f64, t12959: f64, t12962: f64, t12965: f64, t12967: f64, t12971: f64, t12985: f64, t12989: f64, t12993: f64, t12995: f64, t13002: f64, t13005: f64, t13010: f64) -> f64 {
    let t14784 = 0.46308888888888888888e0_f64 * t12998;
    let t14785 = 0.16068111111111111111e1_f64 * t12974;
    let t14791 = -0.309885e1_f64 * t12959 + 0.20839e0_f64 * t12962 - 0.62517e0_f64 * t12965 - 0.41678000000000000001e0_f64 * t12967 - 0.157790625e0_f64 * t12971 + 0.3529725e1_f64 * t12993 + 0.6311625e0_f64 * t12995 - t14784 - t14785 - 0.104195e0_f64 * t13002 + 0.62517e0_f64 * t13005 + 0.264729375e1_f64 * t13010 - 0.103295e1_f64 * t12985 + 0.309885e1_f64 * t12989;
    t14791
}
