//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 855/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk855(t45228: f64, t44713: f64, t4820: f64, t7513: f64, t11780: f64, t23000: f64, t2617: f64, t2679: f64, t9805: f64, t23477: f64, t44708: f64, t43432: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45229 = 0.19171462976960374838e0_f64 * t45228;
    let t45232 = 0.79445533226334281487e-1_f64 * t7513 * t4820 * t44713;
    let t45234 = t23000 * t11780 * t2617;
    let t45238 = t9805 * t11780 * t2679;
    let t45242 = 0.23833659967900284446e0_f64 * t23477 * t4820 * t44708;
    let t45243 = 0.11916829983950142223e0_f64 * t43432;
    (t45229, t45232, t45234, t45238, t45242, t45243)
}
