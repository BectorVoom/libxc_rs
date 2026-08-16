//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 935/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk935(t1268: f64, t2349: f64, t2360: f64, t2923: f64, t10845: f64, t2347: f64, t3886: f64, t904: f64, t10864: f64, t2924: f64, t2697: f64, t3750: f64) -> (f64, f64, f64, f64, f64) {
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
