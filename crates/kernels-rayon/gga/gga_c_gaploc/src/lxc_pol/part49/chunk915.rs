//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 915/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk915(t12914: f64, t1562: f64, t4614: f64, t12806: f64, t4540: f64, t4673: f64, t3116: f64, t7995: f64, t1445: f64, t597: f64, t2787: f64, t9127: f64) -> (f64, f64, f64, f64, f64) {
    let t41769 = t1562 * t4614 * t12914;
    let t41773 = 0.14300195980740170667e1_f64 * t4540 * t4673 * t12806;
    let t41774 = t7995 * t3116;
    let t41777 = 0.11502877786176224903e2_f64 * t597 * t1445 * t41774;
    let t41778 = t2787 * t9127;
    (t41769, t41773, t41774, t41777, t41778)
}
