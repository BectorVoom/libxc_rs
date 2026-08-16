//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1039/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1039(t1445: f64, t3209: f64, t813: f64, t8528: f64, t10915: f64, t22242: f64, t43598: f64, t2684: f64, t43486: f64, t7585: f64, t10930: f64, t10931: f64) -> (f64, f64, f64, f64) {
    let t43787 = 0.92023022289409799224e1_f64 * t813 * t1445 * t8528 * t3209;
    let t43790 = 0.21450293971110256001e1_f64 * t22242 * t10915 * t43598;
    let t43793 = 0.87421871174939309262e2_f64 * t2684 * t7585 * t43486;
    let t43800 = 0.55213813373645879534e2_f64 * t10930 * t10931 * t43486;
    (t43787, t43790, t43793, t43800)
}
