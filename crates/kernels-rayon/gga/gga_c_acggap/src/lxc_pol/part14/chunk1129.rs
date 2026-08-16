//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1129/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1129(t142: f64, t6293: f64, t8888: f64, t30120: f64, t9649: f64, t4680: f64, t7413: f64, t9648: f64, t1815: f64, t1983: f64, t30127: f64, t7586: f64) -> (f64, f64, f64, f64) {
    let t39632 = t8888 * t142 * t6293;
    let t39640 = t30120 * t9649;
    let t39643 = t7413 * t4680 * t9648;
    let t39647 = t30127 * t7586 * t1983 * t1815;
    (t39632, t39640, t39643, t39647)
}
