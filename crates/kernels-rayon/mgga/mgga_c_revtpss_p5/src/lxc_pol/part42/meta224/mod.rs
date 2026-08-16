//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk868;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta224(t5978: f64, t827: f64, t828: f64, t124: f64, t5962: f64, t800: f64, t5966: f64, t2477: f64, t190: f64, t5825: f64, t706: f64, t5819: f64, t2611: f64, t2498: f64, t2518: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t2621: f64, t2628: f64, t2632: f64, t5924: f64, t5925: f64, t5927: f64, t5943: f64, t5945: f64, t5947: f64, t5948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5980, t5984, t5985, t5988, t5989, t5993, t5999, t6001, t6002) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk868(t5978, t827, t828, t124, t5962, t800, t5966, t2477, t190, t5825, t706, t5819);
        let (t6004, t6005) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk869(t2611, t6002, t2498, t2518, t2522, t2562, t2569, t2579, t2587, t2610, t2621, t2628, t2632, t5924, t5925, t5927, t5943, t5945, t5947, t5948, t6001);
    (t5980, t5984, t5985, t5988, t5989, t5993, t5999, t6001, t6002, t6004, t6005)
}
