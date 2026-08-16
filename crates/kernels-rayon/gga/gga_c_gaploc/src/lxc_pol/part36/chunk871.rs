//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 871/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk871(t10215: f64, t555: f64, t188: f64, t3377: f64, t10485: f64, t9333: f64, t31139: f64, t544: f64, t986: f64, t2386: f64, t10525: f64, t10526: f64, t41965: f64) -> (f64, f64, f64, f64, f64) {
    let t42212 = t555 * t10215;
    let t42214 = t188 * t42212 * t3377;
    let t42216 = t10485 * t9333;
    let t42219 = t544 * t31139 * t986;
    let t42221 = 0.25025342966295298669e1_f64 * t42219 * t2386;
    let t42224 = 0.21450293971110256001e1_f64 * t10525 * t10526 * t41965;
    (t42212, t42214, t42216, t42221, t42224)
}
