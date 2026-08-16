//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1109;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta329(t1207: f64, t456: f64, t487: f64, t1269: f64, t3566: f64, t1203: f64, t3565: f64, t3552: f64, t1208: f64, t3551: f64, t1209: f64, t3727: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12627, t12628, t12633, t12640, t12641, t12654, t12657, t12658, t12666) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1109(t1207, t456, t487, t1269, t3566, t1203, t3565, t3552, t1208, t3551, t1209, t3727);
    (t12627, t12628, t12633, t12640, t12641, t12654, t12657, t12658, t12666)
}
