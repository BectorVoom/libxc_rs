//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1089/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1089(t2068: f64, t4680: f64, t8738: f64, t1181: f64, t20987: f64, t7351: f64, t7575: f64, t20992: f64, t7426: f64, t20138: f64, t599: f64, t7413: f64) -> (f64, f64, f64, f64) {
    let t34937 = t2068 * t4680 * t8738;
    let t34941 = t7575 * t1181 * t7351 * t20987;
    let t34945 = t7426 * t1181 * t7351 * t20992;
    let t34946 = 0.18868855373762491241e-2_f64 * t34945;
    let t34949 = t7413 * t1181 * t599 * t20138;
    (t34937, t34941, t34946, t34949)
}
