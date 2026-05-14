//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 431/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk431<F: Float>(t107: F, t622: F, t1656: F, t290: F, t1587: F, t338: F, t1614: F, t321: F, t570: F) -> (F, F, F, F, F) {
    let t5058 = t622 * t107;
    let t5061 = t290 * t1656;
    let t5098 = t338 * t1587;
    let t5126 = t338 * t1614;
    let t5144 = t570 * t321;
    (t5058, t5061, t5098, t5126, t5144)
}
