//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 967/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk967(t17723: f64, t894: f64, t1506: f64, t19: f64, t4356: f64, t15236: f64, t4305: f64, t5268: f64, t11671: f64, t14885: f64, t14887: f64, t14889: f64, t17338: f64, t17342: f64, t17346: f64, t17350: f64, t17354: f64, t17358: f64, t8885: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17724 = t894 * t17723;
    let t17727 = t19 * t1506;
    let t17728 = t17727 * t4356;
    let t17729 = t15236 * t17728;
    let t17733 = 0.17544670192365612213e1_f64 * t4305 * t5268;
    let t17744 = -t8885 - 0.23744444444444444444e-1_f64 * t11671 + 0.11872222222222222222e-1_f64 * t14885 - 0.35616666666666666666e-1_f64 * t14887 + 0.17808333333333333333e-1_f64 * t14889 - 0.19787037037037037037e-1_f64 * t17338 + 0.71233333333333333332e-1_f64 * t17342 - 0.35616666666666666666e-1_f64 * t17346 - 0.10685e0_f64 * t17350 + 0.10685e0_f64 * t17354 - 0.17808333333333333333e-1_f64 * t17358;
    (t17724, t17727, t17728, t17729, t17733, t17744)
}
