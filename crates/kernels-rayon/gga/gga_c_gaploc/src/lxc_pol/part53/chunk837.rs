//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 837/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk837(t12806: f64, t4540: f64, t4673: f64, t3116: f64, t7995: f64, t1445: f64, t597: f64, t2787: f64, t9127: f64, t12894: f64, t18658: f64, t3085: f64, t8097: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41773 = 0.14300195980740170667e1_f64 * t4540 * t4673 * t12806;
    let t41774 = t7995 * t3116;
    let t41777 = 0.11502877786176224903e2_f64 * t597 * t1445 * t41774;
    let t41778 = t2787 * t9127;
    let t41781 = 0.11502877786176224903e2_f64 * t597 * t1445 * t41778;
    let t41783 = 0.21450293971110256001e1_f64 * t18658 * t12894;
    let t41784 = t8097 * t3085;
    (t41773, t41774, t41777, t41778, t41781, t41783, t41784)
}
