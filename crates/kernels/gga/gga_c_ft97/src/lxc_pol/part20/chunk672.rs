//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 672/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk672<F: Float>(t14647: F, t2349: F, t2665: F, t446: F, t1212: F, t2347: F, t10409: F, t1882: F, t4053: F, t4129: F, t668: F, t505: F, t1934: F, t4051: F, t13352: F, t2857: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14648 = t14647 * t2349;
    let t14649 = t2665 * t14648;
    let t14650 = t446 * t14649;
    let t14652 = t1212 * t2347;
    let t14653 = t14652 * t2349;
    let t14654 = t10409 * t14653;
    let t14655 = t446 * t14654;
    let t14657 = t1882 * t4053;
    let t14658 = t14657 / 27.0;
    let t14659 = t4129 * t668;
    let t14660 = t14659 * t505;
    let t14661 = t2665 * t14660;
    let t14662 = t446 * t14661;
    let t14664 = t4051 * t1934;
    let t14665 = t2665 * t14664;
    let t14666 = t446 * t14665;
    let t14668 = t2857 * t13352;
    (t14648, t14650, t14653, t14655, t14657, t14658, t14660, t14662, t14664, t14666, t14668)
}
