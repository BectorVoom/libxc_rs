//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 818/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk818(t13262: f64, t6313: f64, t13327: f64, t13277: f64, t11271: f64, t2268: f64, t2349: f64, t11187: f64, t2317: f64, t6525: f64, t11254: f64, t2293: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44437 = 0.7588001769513639893e-1_f64 * t6313 * t13262;
    let t44439 = 0.37940008847568199465e-1_f64 * t6313 * t13327;
    let t44443 = 0.22764005308540919679e0_f64 * t6313 * t13277;
    let t44457 = 0.85365019907028448797e-1_f64 * t2268 * t11271 * t2349;
    let t44468 = t6525 * t11187 * t2317;
    let t44469 = 0.11856252764865062333e-2_f64 * t44468;
    let t44470 = t11254 * t2293;
    (t44437, t44439, t44443, t44457, t44469, t44470)
}
