//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 918/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk918(t1614: f64, t2426: f64, t3771: f64, t679: f64, t4977: f64, t694: f64, t3724: f64, t5049: f64, t237: f64, t2382: f64, t4985: f64, t1771: f64, t4966: f64) -> (f64, f64, f64, f64, f64) {
    let t66096 = t2426 * t1614;
    let t66098 = t3771 * t66096 * t679;
    let t66115 = t694 * t4977;
    let t66137 = t3724 * t694 * t5049;
    let t66154 = t2382 * t4985 * t237;
    let t66197 = t1771 * t4966;
    (t66098, t66115, t66137, t66154, t66197)
}
