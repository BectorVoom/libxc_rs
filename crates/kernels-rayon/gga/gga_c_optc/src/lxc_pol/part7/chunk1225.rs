//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1225/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1225(t2704: f64, t7845: f64, t284: f64, t7906: f64, t928: f64, t2800: f64, t8177: f64, t2629: f64, t7274: f64, t930: f64, t2634: f64, t7373: f64, t857: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25302 = t2704 * t7845;
    let t25305 = t928 * t7906 * t284;
    let t25308 = t8177 * t2800;
    let t25313 = t930 * t7274 * t2629;
    let t25316 = t930 * t7274 * t2634;
    let t25320 = t857 * t7373;
    (t25302, t25305, t25308, t25313, t25316, t25320)
}
