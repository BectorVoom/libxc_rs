//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk776;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk777;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta179(t1412: f64, t72: f64, t245: f64, t125: f64, t1398: f64, t1353: f64, t543: f64, t159: f64, t550: f64, t216: f64, t124: f64, t3829: f64, t800: f64, t1376: f64, t2689: f64, t1413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3935, t3936) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk776(t1412, t72, t245);
        let (t3937, t3938) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk777(t125, t1398, t1353, t543);
        let (t3940, t3943, t3944, t3946, t3950, t3951) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk778(t3937, t3938, t3936, t159, t550, t216, t124, t3829, t800, t1376, t2689, t1353, t1413);
    (t3935, t3936, t3938, t3940, t3943, t3944, t3946, t3950, t3951)
}
