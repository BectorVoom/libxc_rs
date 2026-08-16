//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2094/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2094(t16781: f64, t16803: f64, t225: f64, t10054: f64, t5585: f64, t13176: f64, t1499: f64, t1523: f64, t1525: f64, t16673: f64, t16679: f64, t16754: f64, t16756: f64, t16759: f64, t16762: f64, t255: f64, t2617: f64, t4162: f64, t4166: f64, t4286: f64, t4291: f64, t4296: f64, t4298: f64, t5645: f64, t5648: f64, t5653: f64, t812: f64, t861: f64) -> (f64, f64, f64, f64) {
    let t16804 = t16781 + t16803;
    let t16805 = t16804 * t225;
    let t16811 = t10054 * t5585;
    let t16814 = -2.0_f64 * t13176 * t1523 + 2.0_f64 * t1499 * t4298 + 2.0_f64 * t1525 * t4162 - t16673 * t861 - 2.0_f64 * t16679 * t812 - t16754 * t812 - t16756 * t812 - 2.0_f64 * t16759 * t4291 - 2.0_f64 * t16762 * t4291 + t16805 * t255 + 2.0_f64 * t16811 * t812 + 2.0_f64 * t2617 * t5645 - 2.0_f64 * t2617 * t5648 - t2617 * t5653 - 2.0_f64 * t4166 * t4286 - 2.0_f64 * t4166 * t4296;
    (t16804, t16805, t16811, t16814)
}
