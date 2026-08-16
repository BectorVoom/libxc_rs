//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2371;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2372;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta550(t17730: f64, t5051: f64, t3626: f64, t3566: f64, t489: f64, t17728: f64, t1121: f64, t1774: f64, t3584: f64, t471: f64, t5351: f64, t3720: f64, t13142: f64, t17708: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17731, t17732, t17735, t17736) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2371(t17730, t5051, t3626, t3566, t489, t17728);
        let t17737 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2372(t1121, t1774);
        let (t17738, t17739, t17742, t17743, t17744, t17747) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2373(t17730, t17737, t3626, t3584, t471, t5351, t3720, t13142, t17708);
    (t17731, t17732, t17735, t17736, t17737, t17738, t17739, t17742, t17743, t17744, t17747)
}
