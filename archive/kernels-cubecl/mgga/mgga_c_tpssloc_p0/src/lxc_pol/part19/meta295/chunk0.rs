//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1076/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1076<F: Float>(t649: F, t671: F, t157: F, t9929: F, t2379: F, t262: F, t9897: F, t2570: F, t67: F, t792: F, t131: F, t9558: F) -> (F, F, F, F, F, F) {
    let t12734 = t649 * t671;
    let t12908 = t9929 * t157;
    let t12935 = t2379 * t262;
    let t12939 = t9897 * t157;
    let t12997 = t2570 * t67;
    let t12998 = t792 * t12997;
    let t13004 = t9558 * t131;
    (t12734, t12908, t12935, t12939, t12998, t13004)
}
