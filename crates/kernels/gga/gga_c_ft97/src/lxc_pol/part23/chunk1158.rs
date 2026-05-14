//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1158/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1158<F: Float>(t24330: F, t25049: F, t28667: F, t14763: F, t6248: F, t111837: F, t28676: F, t109314: F, t28552: F, t28616: F, t109117: F, t6256: F, t2691: F, t28557: F, t4113: F, t109124: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112055 = 0.13335600218518518519e0 * t25049 * t24330 * t28667;
    let t112060 = t14763 * t6248;
    let t112071 = t28676 * t111837;
    let t112133 = t28552 * t109314;
    let t112137 = 0.13335600218518518519e0 * t25049 * t24330 * t28616;
    let t112138 = t6256 * t109117;
    let t112156 = t2691 * t28557;
    let t112159 = t4113 * t111837;
    let t112196 = t6256 * t109124;
    (t112055, t112060, t112071, t112133, t112137, t112138, t112156, t112159, t112196)
}
