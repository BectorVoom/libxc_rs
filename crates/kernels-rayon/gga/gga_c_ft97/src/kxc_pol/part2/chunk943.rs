//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 943/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk943(t14635: f64, t1882: f64, t4041: f64, t4034: f64, t13309: f64, t2857: f64, t446: f64, t10758: f64, t13315: f64, t1212: f64, t2360: f64, t2349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14636 = t14635 / 27.0_f64;
    let t14637 = t1882 * t4041;
    let t14638 = 2.0_f64 / 27.0_f64 * t14637;
    let t14639 = t1882 * t4034;
    let t14640 = 2.0_f64 / 81.0_f64 * t14639;
    let t14641 = t2857 * t13309;
    let t14642 = t446 * t14641;
    let t14644 = t10758 * t13315;
    let t14645 = t446 * t14644;
    let t14647 = t1212 * t2360;
    let t14648 = t14647 * t2349;
    (t14636, t14637, t14638, t14639, t14640, t14642, t14645, t14648)
}
