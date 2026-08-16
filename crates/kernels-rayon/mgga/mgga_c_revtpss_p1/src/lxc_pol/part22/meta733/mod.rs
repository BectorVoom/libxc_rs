//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta733 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2792;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta733(t2693: f64, t2710: f64, t9732: f64, t2430: f64, t853: f64, t2682: f64, t820: f64, t823: f64, t2751: f64, t10886: f64, t808: f64, t10292: f64, t65: f64, t235: f64, t826: f64, t225: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t40535, t40555, t40593) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2792(t2693, t2710, t9732, t2430, t853, t2682, t820, t823);
        let (t40594, t40600, t40604, t40607, t40609) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2793(t2751, t40593, t10886, t40555, t808, t10292, t65, t235, t2710, t826, t225, t785);
    (t40535, t40593, t40594, t40600, t40604, t40607, t40609)
}
