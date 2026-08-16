//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 894/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk894(t23477: f64, t44708: f64, t4820: f64, t43432: f64, t2617: f64, t3630: f64, t7803: f64, t11894: f64, t1445: f64, t2087: f64, t2530: f64, t313: f64, t45001: f64) -> (f64, f64, f64, f64, f64) {
    let t45242 = 0.23833659967900284446e0_f64 * t23477 * t4820 * t44708;
    let t45243 = 0.11916829983950142223e0_f64 * t43432;
    let t45246 = t7803 * t3630 * t2617;
    let t45247 = 0.19171462976960374838e0_f64 * t45246;
    let t45251 = 0.69017266717057349418e1_f64 * t2087 * t1445 * t11894 * t2530;
    let t45252 = t313 * t45001;
    (t45242, t45243, t45247, t45251, t45252)
}
