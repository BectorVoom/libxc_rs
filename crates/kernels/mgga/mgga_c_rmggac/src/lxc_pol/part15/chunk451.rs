//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 451/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk451<F: Float>(t107: F, t622: F, t1656: F, t290: F, t1587: F, t338: F, t1614: F, t321: F, t570: F) -> (F, F, F, F, F) {
    let t5058 = t622 * t107;
    let t5061 = t290 * t1656;
    let t5098 = t338 * t1587;
    let t5126 = t338 * t1614;
    let t5144 = t570 * t321;
    (t5058, t5061, t5098, t5126, t5144)
}
