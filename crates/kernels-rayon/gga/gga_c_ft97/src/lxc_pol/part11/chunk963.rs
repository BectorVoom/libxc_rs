//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 963/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk963(t37795: f64, t37798: f64, t37800: f64, t37802: f64, t37806: f64, t37808: f64, t37812: f64, t37816: f64, t37824: f64, t37828: f64, t40033: f64, t1701: f64, t2071: f64, t7883: f64) -> (f64, f64) {
    let t40036 = 0.35561600582716049384e0_f64 * t37795 + 0.44452000728395061732e-1_f64 * t37798 - 0.2370773372181069959e0_f64 * t37800 - 0.55318045350891632375e0_f64 * t37802 + 0.66678001092592592595e-1_f64 * t37806 - 0.17780800291358024692e0_f64 * t37808 - 0.65196267734979423872e0_f64 * t37812 - 0.19756444768175582992e0_f64 * t37816 - t40033 + 0.12930593100770919068e2_f64 * t37824 - 0.30424924942990397807e1_f64 * t37828;
    let t40046 = t1701 * t7883 * t2071;
    (t40036, t40046)
}
