//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 859/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk859(t13352: f64, t3910: f64, t1091: f64, t2373: f64, t9896: f64, t1131: f64, t2360: f64, t2349: f64, t2493: f64, t3930: f64, t9707: f64, t2: f64, t3821: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13353 = t3910 * t13352;
    let t13356 = t1091 * t2373;
    let t13357 = t9896 * t13356;
    let t13360 = t1131 * t2360;
    let t13361 = t13360 * t2349;
    let t13362 = t2493 * t13361;
    let t13370 = t9707 * t3930 * t2373;
    let t13373 = t2 * t3821;
    (t13353, t13356, t13357, t13361, t13362, t13370, t13373)
}
