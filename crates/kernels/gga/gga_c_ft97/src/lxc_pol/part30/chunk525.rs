//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 525/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk525<F: Float>(t24482: F, t24537: F, t1445: F, t2399: F, t89: F, t1449: F, t2567: F, t6163: F, t8392: F, t1882: F, t6101: F, t6156: F, t6085: F, t6094: F, t1424: F, t761: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24628 = 4.0 / 27.0 * t24482;
    let t24642 = 2.0 / 27.0 * t24537;
    let t24658 = 4.0 / 27.0 * t89 * t2399 * t1445;
    let t24668 = t2567 * t1449;
    let t24673 = t8392 * t6163;
    let t24690 = t1882 * t6101;
    let t24731 = t1882 * t6156;
    let t24733 = t1882 * t6085;
    let t24735 = t1882 * t6094;
    let t24737 = t761 * t1424;
    (t24628, t24642, t24658, t24668, t24673, t24690, t24731, t24733, t24735, t24737)
}
