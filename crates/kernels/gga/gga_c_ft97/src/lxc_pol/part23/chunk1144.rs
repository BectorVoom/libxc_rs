//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1144/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1144<F: Float>(t28133: F, t8392: F, t1882: F, t28286: F, t1449: F, t41408: F, t38953: F, t6849: F, t6837: F, t761: F, t12001: F, t28222: F, t28142: F, t255: F, t41848: F, t256: F, t28300: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110582 = 2.0 / 27.0 * t8392 * t28133;
    let t110588 = 2.0 / 9.0 * t1882 * t28286;
    let t110612 = t41408 * t1449;
    let t110626 = t38953 * t6849;
    let t110629 = t761 * t6837;
    let t110641 = t12001 * t28222;
    let t110659 = 4.0 / 3.0 * t8392 * t28142;
    let t110660 = t41848 * t255;
    let t110665 = t256 * t28300;
    (t110582, t110588, t110612, t110626, t110629, t110641, t110659, t110660, t110665)
}
