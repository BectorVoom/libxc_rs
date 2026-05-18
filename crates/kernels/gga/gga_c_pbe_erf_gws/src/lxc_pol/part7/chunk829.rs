//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 829/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk829<F: Float>(t4456: F, t6123: F, t6754: F, t6835: F, t945: F, t321: F, t2054: F, t804: F, t810: F, t2182: F, t2429: F, t946: F) -> (F, F, F, F, F) {
    let t6837 = t4456 + t6123 + t6754 + t6835;
    let t6838 = t6837 * t945;
    let t6839 = t321 * t6838;
    let t6841 = t804 * t2054 * t810;
    let t6845 = t2429 * t946 * t2182;
    (t6837, t6838, t6839, t6841, t6845)
}
