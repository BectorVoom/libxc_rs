//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 958/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk958<F: Float>(t1963: F, t2042: F, t22417: F, t22434: F, t22439: F, t22655: F, t22657: F, t22659: F, t22661: F, t22663: F, t22666: F, t22669: F, t22671: F, t22675: F, t1986: F, t6814: F) -> (F, F, F) {
    let t22676 = t2042 * t1963;
    let t22677 = 120.0 * t22676;
    let t22678 = -t22655 - t22417 + t22434 - t22439 + t22657 - t22659 + t22661 - t22663 + t22666 + t22669 - t22671 - t22675 + t22677;
    let t22680 = t1986 * t6814;
    (t22677, t22678, t22680)
}
