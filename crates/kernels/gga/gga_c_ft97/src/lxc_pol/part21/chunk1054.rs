//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1054/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1054<F: Float>(t22619: F, t25653: F, t415: F, t1608: F, t17839: F, t5596: F, t25759: F, t420: F, t6449: F, t92557: F, t5611: F, t92433: F, t5566: F, t65750: F, t22522: F, t22572: F, t25760: F) -> (F, F, F, F, F, F, F, F, F) {
    let t100753 = 0.29693535778629056444e-3 * t22619 * t415 * t25653;
    let t100763 = t1608 * t5596 * t17839;
    let t100784 = t420 * t25759;
    let t100800 = t92557 * t6449;
    let t100801 = t5611 * t100800;
    let t100806 = t92433 * t6449;
    let t100808 = 0.1134997482304526749e-1 * t5611 * t100806;
    let t100843 = t1608 * t5566 * t65750;
    let t100848 = t22522 * t22572 * t25760;
    (t100753, t100763, t100784, t100800, t100801, t100806, t100808, t100843, t100848)
}
