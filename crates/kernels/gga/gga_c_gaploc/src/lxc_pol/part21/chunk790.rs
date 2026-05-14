//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 790/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk790<F: Float>(t1445: F, t8012: F, t7963: F, t4529: F, t986: F, t1328: F, t2778: F, t2787: F, t8000: F, t8004: F, t447: F, t7995: F, t1265: F, t2854: F, t2765: F, t524: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8118 = t1445 * t8012;
    let t8121 = t1445 * t7963;
    let t8124 = t4529 * t986;
    let t8125 = t8124 * t1328;
    let t8126 = t1445 * t8125;
    let t8131 = t2778 * t1328;
    let t8132 = t1445 * t8131;
    let t8135 = t2787 * t1328;
    let t8136 = t1445 * t8135;
    let t8139 = t1445 * t8000;
    let t8142 = t1445 * t8004;
    let t8147 = t7995 * t447;
    let t8148 = t1445 * t8147;
    let t8151 = t2854 * t1265;
    let t8152 = t1445 * t8151;
    let t8155 = t524 * t2765;
    (t8118, t8121, t8124, t8126, t8132, t8136, t8139, t8142, t8148, t8152, t8155)
}
