//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 961/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk961<F: Float>(t18852: F, t898: F, t900: F, t1268: F, t992: F, t505: F, t14514: F, t10864: F, t668: F, t904: F, t14519: F, t4357: F, t4370: F) -> (F, F, F, F, F) {
    let t18854 = t898 * t900 * t18852;
    let t18857 = t992 * t1268;
    let t18858 = t18857 * t505;
    let t18859 = t14514 * t18858;
    let t18862 = t10864 * t668;
    let t18864 = t18862 * t18857 * t904;
    let t18867 = t14519 * t18858;
    let t18871 = t898 * t4357 * t4370;
    (t18854, t18859, t18864, t18867, t18871)
}
