//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1224/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1224(t15730: f64, t5002: f64, t15734: f64, t5024: f64, t11818: f64, t248: f64, t3515: f64, t6230: f64, t11789: f64, t1227: f64, t5979: f64, t3540: f64, t6165: f64) -> (f64, f64, f64, f64, f64) {
    let t65605 = t5002 * t15730;
    let t65628 = t5024 * t15734;
    let t65632 = t3515 * t248 * t11818 * t6230;
    let t65647 = t1227 * t248 * t11789 * t5979;
    let t65664 = t6165 * t3540;
    (t65605, t65628, t65632, t65647, t65664)
}
