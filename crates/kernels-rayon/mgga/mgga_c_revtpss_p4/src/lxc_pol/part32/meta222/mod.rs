//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk951;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta222(t114: f64, t5915: f64, t655: f64, t2335: f64, t4261: f64, t5892: f64, t69: f64, t508: f64, t4303: f64, t4306: f64, t2498: f64, t2518: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t2628: f64, t2632: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t5916, t5920) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk951(t114, t5915, t655, t2335, t4261, t5892, t69);
        let (t5921, t5924, t5925, t5926) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk952(t508, t5920, t4303, t4306, t2498, t2518, t2522, t2562, t2569, t2579, t2587, t2610, t2628, t2632);
    (t5916, t5920, t5921, t5924, t5925, t5926)
}
