//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 614/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk614<F: Float>(t3822: F, t576: F, t3725: F, t3730: F, t3735: F, t3740: F, t338: F, t1096: F, t3565: F, t1125: F, t3265: F) -> (F, F, F, F, F, F) {
    let t3823 = t576 * t3822;
    let t3828 = 0.16414765573575218917e-4 * t3725 - 0.23485962392041415794e-4 * t3730 - 0.34197428278281706076e-6 * t3735 + 0.14678726495025884871e-5 * t3740;
    let t3829 = t3828 * t338;
    let t3830 = t3565 * t1096;
    let t3831 = t3265 * t1125;
    let t3832 = t1125 * t1096;
    (t3823, t3828, t3829, t3830, t3831, t3832)
}
