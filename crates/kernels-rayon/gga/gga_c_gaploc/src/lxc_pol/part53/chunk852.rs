//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 852/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk852(t2386: f64, t42149: f64, t204: f64, t2476: f64, t41738: f64, t10615: f64, t1423: f64, t3129: f64, t12871: f64, t8155: f64, t8158: f64, t40372: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42151 = 0.53625734927775640005e1_f64 * t42149 * t2386;
    let t42154 = 0.92023022289409799224e1_f64 * t2476 * t204 * t41738;
    let t42156 = t10615 * t1423 * t3129;
    let t42157 = 0.17875244975925213335e0_f64 * t42156;
    let t42159 = 0.10725146985555128001e1_f64 * t8155 * t12871;
    let t42161 = 0.10725146985555128001e1_f64 * t8158 * t12871;
    let t42168 = 0.63904876589867916127e-1_f64 * t40372;
    (t42151, t42154, t42157, t42159, t42161, t42168)
}
