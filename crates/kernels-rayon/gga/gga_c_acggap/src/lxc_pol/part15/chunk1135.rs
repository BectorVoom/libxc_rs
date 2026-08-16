//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1135/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1135(t4680: f64, t7413: f64, t9648: f64, t1815: f64, t1983: f64, t30127: f64, t7586: f64, t31350: f64, t6343: f64, t30811: f64, t6347: f64, t142: f64, t2060: f64, t5674: f64, t604: f64) -> (f64, f64, f64, f64, f64) {
    let t39643 = t7413 * t4680 * t9648;
    let t39647 = t30127 * t7586 * t1983 * t1815;
    let t39649 = t31350 * t6343;
    let t39653 = t30811 * t6347;
    let t39658 = t2060 * t142 * t604 * t5674;
    (t39643, t39647, t39649, t39653, t39658)
}
