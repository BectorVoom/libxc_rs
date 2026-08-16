//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta682 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2667;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta682(t5854: f64, t607: f64, t10355: f64, t5819: f64, t606: f64, t4186: f64, t4201: f64, t2275: f64, t5825: f64, t18281: f64, t48: f64, t10368: f64, t4210: f64, t2282: f64, t60: f64, t10379: f64, t1480: f64, t4211: f64, t4214: f64, t44: f64, t56: f64, t5835: f64, t5838: f64, t5843: f64, t614: f64, t620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21727, t21732, t21733, t21736, t21741, t21742, t21745, t21754) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2667(t5854, t607, t10355, t5819, t606, t4186, t4201, t2275, t5825, t18281, t48, t10368);
        let (t21761, t21768) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2668(t21754, t606, t4186, t4210, t2282, t5825, t18281, t60, t10379, t1480, t21733, t21736, t21742, t21745, t4211, t4214, t44, t56, t5835, t5838, t5843, t614, t620);
    (t21727, t21732, t21733, t21736, t21741, t21742, t21745, t21754, t21761, t21768)
}
