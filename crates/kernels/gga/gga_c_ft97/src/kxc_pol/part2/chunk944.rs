//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 944/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk944<F: Float>(t14648: F, t2665: F, t446: F, t1212: F, t2347: F, t2349: F, t10409: F, t1882: F, t4053: F, t4129: F, t668: F, t505: F) -> (F, F, F, F, F, F) {
    let t14649 = t2665 * t14648;
    let t14650 = t446 * t14649;
    let t14652 = t1212 * t2347;
    let t14653 = t14652 * t2349;
    let t14654 = t10409 * t14653;
    let t14655 = t446 * t14654;
    let t14657 = t1882 * t4053;
    let t14658 = t14657 / F::new(27.0);
    let t14659 = t4129 * t668;
    let t14660 = t14659 * t505;
    (t14650, t14653, t14655, t14657, t14658, t14660)
}
