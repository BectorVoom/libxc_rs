//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk918;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta239(t3390: f64, t6442: f64, t3394: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t1132: f64, t3407: f64, t1139: f64, t3417: f64, t6421: f64, t141: f64, t1145: f64, t6425: f64, t6429: f64, t3402: f64, t3414: f64, t5093: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6443, t6449, t6450, t6456, t6458, t6461) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk918(t3390, t6442, t3394, t5044, t6423, t6427, t6431, t1132, t3407, t1139, t3417, t6421);
        let (t6462, t6464, t6465, t6467, t6468, t6470) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk919(t141, t6461, t1145, t6425, t6429, t3402, t3414, t5044, t5093, t6423, t6427, t6431, t6443, t6450, t6456, t6458);
    (t6443, t6449, t6450, t6456, t6458, t6461, t6462, t6464, t6465, t6467, t6468, t6470)
}
