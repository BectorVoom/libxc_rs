//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1320/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1320<F: Float>(t56955: F, t57229: F, t57521: F, t57528: F, t25093: F, t55901: F, t894: F, t2596: F, t55906: F, t25001: F, t4776: F, t8201: F) -> (F, F, F, F, F) {
    let t57530 = t56955 + t57229 + t57521 + t57528;
    let t57537 = t894 * t25093 * t55901;
    let t57541 = t894 * t2596 * t55906;
    let t57545 = t894 * t25001 * t55901;
    let t57554 = t8201 * t4776;
    (t57530, t57537, t57541, t57545, t57554)
}
