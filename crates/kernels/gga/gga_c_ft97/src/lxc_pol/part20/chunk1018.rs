//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1018/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1018<F: Float>(t1443: F, t9895: F, t1431: F, t3281: F, t6061: F, t761: F, t6105: F, t8232: F, t24564: F, t24571: F, t8392: F, t1882: F, t24714: F, t24719: F, t24749: F, t6172: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t97793 = t9895 * t1443;
    let t97809 = 28.0 / 81.0 * t3281 * t1431;
    let t97810 = t761 * t6061;
    let t97815 = t8232 * t6105;
    let t97817 = t761 * t24564;
    let t97831 = t8392 * t24571;
    let t97841 = t1882 * t24714;
    let t97843 = t1882 * t24719;
    let t97861 = t8392 * t24749;
    let t97870 = t8232 * t6172;
    (t97793, t97809, t97810, t97815, t97817, t97831, t97841, t97843, t97861, t97870)
}
