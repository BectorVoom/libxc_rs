//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 938/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk938(t76985: f64, t2144: f64, t3351: f64, t3352: f64, t9524: f64, t70929: f64, t74419: f64, t74421: f64, t74426: f64, t74432: f64, t74446: f64, t74450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76986 = 0.25538759935978703638e-4_f64 * t76985;
    let t76997 = t3351 * t3352 * t2144 * t9524;
    let t76998 = 0.38308139903968055457e-4_f64 * t76997;
    let t76999 = 0.99317399751028291929e-5_f64 * t70929;
    let t77004 = 0.3192344991997337955e-4_f64 * t74419;
    let t77005 = 0.85129199786595678799e-5_f64 * t74421;
    let t77006 = 0.2553875993597870364e-4_f64 * t74426;
    let t77007 = 0.85129199786595678799e-5_f64 * t74432;
    let t77011 = 0.5107751987195740728e-4_f64 * t74446;
    let t77012 = 0.5107751987195740728e-4_f64 * t74450;
    (t76986, t76998, t76999, t77004, t77005, t77006, t77007, t77011, t77012)
}
