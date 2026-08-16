//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 909/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk909(t3359: f64, t6052: f64, t11352: f64, t6036: f64, t1098: f64, t5983: f64, t11243: f64, t5992: f64, t11265: f64, t1128: f64, t6031: f64, t1147: f64, t6063: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18643 = t6052 * t3359;
    let t18650 = t6036 * t11352;
    let t18686 = t5983 * t1098;
    let t18746 = t11243 * t5992;
    let t18754 = t11265 * t5992;
    let t18840 = t6031 * t1128;
    let t18899 = t6063 * t1147;
    (t18643, t18650, t18686, t18746, t18754, t18840, t18899)
}
