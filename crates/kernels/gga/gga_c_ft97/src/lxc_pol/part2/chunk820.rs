//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 820/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk820<F: Float>(t14632: F, t446: F, t1882: F, t4038: F, t4041: F, t4034: F, t13309: F, t2857: F, t10758: F, t13315: F, t1212: F, t2360: F, t2349: F, t2665: F, t2347: F, t10409: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14633 = t446 * t14632;
    let t14635 = t1882 * t4038;
    let t14636 = t14635 / 27.0;
    let t14637 = t1882 * t4041;
    let t14638 = 2.0 / 27.0 * t14637;
    let t14639 = t1882 * t4034;
    let t14640 = 2.0 / 81.0 * t14639;
    let t14641 = t2857 * t13309;
    let t14642 = t446 * t14641;
    let t14644 = t10758 * t13315;
    let t14645 = t446 * t14644;
    let t14647 = t1212 * t2360;
    let t14648 = t14647 * t2349;
    let t14649 = t2665 * t14648;
    let t14650 = t446 * t14649;
    let t14652 = t1212 * t2347;
    let t14653 = t14652 * t2349;
    let t14654 = t10409 * t14653;
    (t14633, t14635, t14636, t14637, t14638, t14639, t14640, t14642, t14645, t14648, t14650, t14653, t14654)
}
