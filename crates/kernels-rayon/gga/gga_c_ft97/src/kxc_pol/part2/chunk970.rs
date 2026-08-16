//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 970/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk970(t15010: f64, t15055: f64, t845: f64, t91: f64, t2755: f64, t4226: f64, t856: f64, t2789: f64, t4191: f64, t10631: f64, t1234: f64, t2756: f64) -> (f64, f64, f64, f64) {
    let t15056 = t15010 + t15055;
    let t15058 = t91 * t845 * t15056;
    let t15060 = t2755 * t4226;
    let t15062 = t91 * t15060 * t856;
    let t15065 = t91 * t4191 * t2789;
    let t15069 = t91 * t10631 * t1234 * t2756;
    (t15058, t15062, t15065, t15069)
}
