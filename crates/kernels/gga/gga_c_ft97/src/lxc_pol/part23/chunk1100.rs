//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1100/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1100<F: Float>(t2492: F, t6154: F, t1445: F, t89: F, t9555: F, t1443: F, t9895: F, t1431: F, t3281: F, t6061: F, t761: F, t6105: F, t8232: F, t6172: F, t6189: F, t2399: F, t6150: F) -> (F, F, F, F, F, F, F, F, F) {
    let t97777 = t2492 * t6154;
    let t97790 = 28.0 / 81.0 * t89 * t9555 * t1445;
    let t97793 = t9895 * t1443;
    let t97809 = 28.0 / 81.0 * t3281 * t1431;
    let t97810 = t761 * t6061;
    let t97815 = t8232 * t6105;
    let t97870 = t8232 * t6172;
    let t97872 = t8232 * t6189;
    let t97889 = t89 * t2399 * t6150;
    (t97777, t97790, t97793, t97809, t97810, t97815, t97870, t97872, t97889)
}
