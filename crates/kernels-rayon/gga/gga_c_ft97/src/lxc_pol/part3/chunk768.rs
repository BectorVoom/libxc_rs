//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 768/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk768(t3103: f64, t925: f64, t1564: f64, t446: f64, t3052: f64, t942: f64, t3281: f64, t432: f64, t4454: f64, t7793: f64, t15742: f64, t1866: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15932 = t925 * t3103;
    let t15933 = t1564 * t15932;
    let t15934 = t446 * t15933;
    let t15936 = t3052 * t942;
    let t15937 = t1564 * t15936;
    let t15938 = t3281 * t15937;
    let t15940 = t4454 * t432;
    let t15941 = t7793 * t15940;
    let t15942 = t446 * t15941;
    let t15944 = t1866 * t15742;
    (t15932, t15934, t15936, t15938, t15940, t15942, t15944)
}
