//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1191/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1191<F: Float>(t23573: F, t24657: F, t22015: F, t894: F, t23459: F, t23465: F, t23468: F, t23788: F, t23793: F, t23807: F, t23810: F, t23815: F, t23821: F, t23946: F, t24017: F) -> (F, F) {
    let t24658 = t24657 * t23573;
    let t24660 = t894 * t24658 * t22015;
    let t24663 = t23459 - t23465 + t23468 + t23788 - t23793 - t23807 - t23810 - t23815 - t23821 - t23946 - t24017;
    (t24660, t24663)
}
