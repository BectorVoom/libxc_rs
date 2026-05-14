//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 814/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk814<F: Float>(t10845: F, t2347: F, t3886: F, t904: F, t2360: F, t2923: F, t10864: F, t1268: F, t2924: F, t2697: F, t3750: F, t10304: F, t1095: F, t13540: F, t13571: F, t801: F) -> (F, F, F, F, F, F, F) {
    let t14514 = t10845 * t2347;
    let t14515 = t3886 * t904;
    let t14516 = t14514 * t14515;
    let t14519 = t2923 * t2360;
    let t14520 = t14519 * t14515;
    let t14523 = t10864 * t1268;
    let t14524 = t14523 * t2924;
    let t14532 = t2697 * t3750;
    let t14541 = t10304 * t1095;
    let t14544 = 0.6419148148148148148e-1 * t13540;
    let t14550 = t801 * t13571;
    (t14516, t14520, t14524, t14532, t14541, t14544, t14550)
}
