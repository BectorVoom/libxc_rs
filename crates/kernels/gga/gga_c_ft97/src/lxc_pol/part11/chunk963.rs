//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 963/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk963<F: Float>(t37795: F, t37798: F, t37800: F, t37802: F, t37806: F, t37808: F, t37812: F, t37816: F, t37824: F, t37828: F, t40033: F, t1701: F, t2071: F, t7883: F) -> (F, F) {
    let t40036 = F::cast_from(0.35561600582716049384e0_f64) * t37795 + F::cast_from(0.44452000728395061732e-1_f64) * t37798 - F::cast_from(0.2370773372181069959e0_f64) * t37800 - F::cast_from(0.55318045350891632375e0_f64) * t37802 + F::cast_from(0.66678001092592592595e-1_f64) * t37806 - F::cast_from(0.17780800291358024692e0_f64) * t37808 - F::cast_from(0.65196267734979423872e0_f64) * t37812 - F::cast_from(0.19756444768175582992e0_f64) * t37816 - t40033 + F::cast_from(0.12930593100770919068e2_f64) * t37824 - F::cast_from(0.30424924942990397807e1_f64) * t37828;
    let t40046 = t1701 * t7883 * t2071;
    (t40036, t40046)
}
