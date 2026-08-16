//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3182/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3182<F: Float>(t15495: F, t4997: F, t15492: F, t5019: F, t15591: F, t5002: F, t1174: F, t18237: F, t3431: F, t6187: F, t698: F, t1227: F, t13969: F, t18341: F) -> (F, F, F, F, F, F, F) {
    let t65992 = t15495 * t4997;
    let t65994 = t5019 * t15492;
    let t65996 = t15591 * t4997;
    let t65998 = t5002 * t15492;
    let t66001 = t1174 * t3431 * t18237;
    let t66015 = t1174 * t698 * t6187;
    let t66024 = t1227 * t13969 * t18341;
    (t65992, t65994, t65996, t65998, t66001, t66015, t66024)
}
