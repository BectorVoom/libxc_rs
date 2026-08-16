//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta459(t1041: f64, t19658: f64, t5819: f64, t606: f64) -> (f64, f64) {
        let (t19659, t19661) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1897(t1041, t19658, t5819, t606);
    (t19659, t19661)
}
