//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1168/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1168(t13945: f64, t650: f64, t43295: f64, t43298: f64, t43300: f64, t43302: f64, t43304: f64, t47749: f64, t47752: f64, t47755: f64, t47758: f64, t47764: f64) -> f64 {
    let t47766 = 0.10254034973522965712e-1_f64 * t650 * t13945;
    let t47767 = t43295 - 0.46143157380853345701e-1_f64 * t43298 + t43300 - 0.53833683610995569986e-1_f64 * t43302 - 0.53833683610995569986e-1_f64 * t47749 + 0.10254034973522965712e-1_f64 * t43304 + 0.76905262301422242837e-2_f64 * t47752 + 0.76905262301422242837e-2_f64 * t47755 + 0.76905262301422242837e-2_f64 * t47758 + t47764 - t47766;
    t47767
}
