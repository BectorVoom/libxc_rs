//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 993/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk993(t5393: f64, t824: f64, t840: f64, t871: f64, t875: f64, t2843: f64, t296: f64, t15128: f64, t4181: f64, t1882: f64, t5419: f64, t5381: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19373 = t5393 * t824;
    let t19375 = t840 * t871 * t19373;
    let t19378 = t5393 * t875;
    let t19379 = t2843 * t19378;
    let t19380 = t296 * t19379;
    let t19383 = t15128 * t4181;
    let t19384 = t296 * t19383;
    let t19387 = t1882 * t5419;
    let t19389 = t1882 * t5381;
    (t19375, t19379, t19380, t19383, t19384, t19387, t19389)
}
