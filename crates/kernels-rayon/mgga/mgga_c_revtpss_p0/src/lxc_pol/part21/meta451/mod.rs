//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta451(t13312: f64, t190: f64, t706: f64, t4391: f64, t705: f64, t707: f64, t189: f64, t4186: f64, t606: f64, t4401: f64, t10579: f64, t2411: f64, t4537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14383, t14385, t14386, t14388, t14389, t14390, t14392, t14396, t14397) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1978(t13312, t190, t706, t4391, t705, t707, t189, t4186, t606, t4401, t10579, t2411, t4537);
    (t14383, t14385, t14386, t14388, t14389, t14390, t14392, t14396, t14397)
}
