//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 924/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk924(t2021: f64, t43572: f64, t5974: f64, t10817: f64, t9972: f64, t1445: f64, t3209: f64, t813: f64, t8528: f64, t10915: f64, t22242: f64, t43598: f64) -> (f64, f64, f64, f64) {
    let t43781 = 0.25025342966295298669e1_f64 * t2021 * t43572 * t5974;
    let t43783 = 0.50050685932590597338e1_f64 * t10817 * t9972;
    let t43787 = 0.92023022289409799224e1_f64 * t813 * t1445 * t8528 * t3209;
    let t43790 = 0.21450293971110256001e1_f64 * t22242 * t10915 * t43598;
    (t43781, t43783, t43787, t43790)
}
