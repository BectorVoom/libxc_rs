//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1130/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1130<F: Float>(t13675: F, t4565: F, t4941: F, t935: F, t2672: F) -> (F, F, F, F) {
    let t56766 = t13675 * t4565;
    let t56770 = t4941 * t935;
    let t56771 = t56770 * t2672;
    let t56775 = t4941 * t4941;
    (t56766, t56770, t56771, t56775)
}
