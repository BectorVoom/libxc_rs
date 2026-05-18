//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1058/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1058<F: Float>(t1320: F, t6602: F, t732: F, t9430: F, t1310: F, t6569: F, t3386: F, t6814: F, t1313: F, t193: F, t21989: F, t3305: F, t6838: F) -> (F, F, F, F, F, F) {
    let t28156 = t6602 * t1320;
    let t28175 = t732 * t9430;
    let t28181 = t1310 * t6569;
    let t28255 = t3386 * t6814;
    let t28366 = t193 * t21989 * t1313;
    let t28489 = t3305 * t6838;
    (t28156, t28175, t28181, t28255, t28366, t28489)
}
