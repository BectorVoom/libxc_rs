//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1124/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1124(t6708: f64, t95021: f64, t3408: f64, t7312: f64, t1882: f64, t35073: f64, t1384: f64, t2179: f64, t27191: f64, t34947: f64, t604: f64, t1053: f64, t106623: f64, t12680: f64, t13153: f64, t1391: f64, t140169: f64, t144: f64, t1901: f64, t2142: f64, t2210: f64, t26768: f64, t27329: f64, t32869: f64, t33035: f64, t33040: f64, t35125: f64, t35229: f64, t379: f64, t446: f64, t569: f64, t574: f64, t605: f64) -> (f64, f64, f64, f64) {
    let t148120 = t95021 * t6708;
    let t148132 = t7312 * t3408;
    let t148163 = t1882 * t35073;
    let t148166 = t2179 * t1384 * t27191;
    let t148170 = t604 * t34947;
    let t148178 = t446 * t574 * t2142 * t35125 / 3.0_f64 + t446 * t574 * t605 * t32869 * t1053 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t574 * t1391 * t26768 - 2.0_f64 / 27.0_f64 * t140169 + t1901 * t12680 * t33035 / 9.0_f64 + t1901 * t13153 * t33040 / 9.0_f64 - t446 * t569 * t35229 * t379 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t148163 + 4.0_f64 / 3.0_f64 * t446 * t144 * t148166 + t1901 * t2210 * t148170 * t379 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t106623 * t27329;
    (t148120, t148132, t148166, t148178)
}
