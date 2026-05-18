//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 935/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk935<F: Float>(t1268: F, t2349: F, t2360: F, t2923: F, t10845: F, t2347: F, t3886: F, t904: F, t10864: F, t2924: F, t2697: F, t3750: F) -> (F, F, F, F, F) {
    let t14507 = t2923 * t1268 * t2360 * t2349;
    let t14514 = t10845 * t2347;
    let t14515 = t3886 * t904;
    let t14516 = t14514 * t14515;
    let t14519 = t2923 * t2360;
    let t14520 = t14519 * t14515;
    let t14523 = t10864 * t1268;
    let t14524 = t14523 * t2924;
    let t14532 = t2697 * t3750;
    (t14507, t14516, t14520, t14524, t14532)
}
