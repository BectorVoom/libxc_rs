//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 970/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk970<F: Float>(t1244: F, t40: F, t6524: F, t108: F, t1256: F, t176: F, t203: F, t6599: F, t1320: F, t6602: F, t732: F, t9430: F, t1310: F, t6569: F, t3386: F, t6814: F) -> (F, F, F, F, F, F) {
    let t28141 = t40 * t1244 * t6524;
    let t28145 = t176 * t6599 * t1256 * t108 * t203;
    let t28156 = t6602 * t1320;
    let t28175 = t732 * t9430;
    let t28181 = t1310 * t6569;
    let t28255 = t3386 * t6814;
    (t28141, t28145, t28156, t28175, t28181, t28255)
}
