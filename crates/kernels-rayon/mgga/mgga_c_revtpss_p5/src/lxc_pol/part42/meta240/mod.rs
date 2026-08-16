//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk920;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta240(t1150: f64, t6470: f64, t1131: f64, t3435: f64, t6438: f64, t3433: f64, t3439: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t1744: f64, t1169: f64, t3459: f64, t3466: f64, t5093: f64, t6443: f64, t6450: f64, t6456: f64, t6458: f64, t6462: f64, t6465: f64, t6468: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6471, t6473, t6474, t6476, t6481, t6486) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk920(t1150, t6470, t1131, t3435, t6438, t3433, t3439, t5044, t6423, t6427, t6431, t1744);
        let (t6487, t6502) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk921(t1169, t6486, t3459, t3466, t5044, t5093, t6423, t6427, t6431, t6443, t6450, t6456, t6458, t6462, t6465, t6468);
    (t6471, t6473, t6474, t6476, t6481, t6486, t6487, t6502)
}
