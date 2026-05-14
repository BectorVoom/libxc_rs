//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 823/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk823<F: Float>(t2876: F, t3690: F, t10409: F, t446: F, t3699: F, t2665: F, t2680: F, t4129: F, t824: F, t193: F, t89: F, t2739: F, t4056: F, t1212: F, t2682: F, t7640: F) -> (F, F, F, F, F, F, F) {
    let t14686 = t3690 * t2876;
    let t14687 = t10409 * t14686;
    let t14688 = t446 * t14687;
    let t14690 = t3699 * t2876;
    let t14691 = t2665 * t14690;
    let t14692 = t446 * t14691;
    let t14694 = t2680 * t4129;
    let t14695 = t14694 * t824;
    let t14697 = t89 * t193 * t14695;
    let t14699 = t4056 * t2739;
    let t14701 = t89 * t193 * t14699;
    let t14704 = t7640 * t1212 * t2682;
    (t14686, t14688, t14690, t14692, t14697, t14701, t14704)
}
