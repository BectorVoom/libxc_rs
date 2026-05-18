//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 864/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk864<F: Float>(t3088: F, t37269: F, t419: F, t11269: F, t37311: F, t1748: F, t8130: F, t1739: F, t1725: F, t8103: F, t173: F, t8102: F) -> (F, F, F, F, F, F) {
    let t37715 = t419 * t3088 * t37269;
    let t37718 = t419 * t11269 * t37311;
    let t37720 = t8130 * t1748;
    let t37723 = t8130 * t1739;
    let t37725 = t1725 * t8103;
    let t37728 = t419 * t173 * t8102;
    (t37715, t37718, t37720, t37723, t37725, t37728)
}
