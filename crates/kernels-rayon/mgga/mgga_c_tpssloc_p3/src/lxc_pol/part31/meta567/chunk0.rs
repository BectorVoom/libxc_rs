//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1798/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1798(t25097: f64, t81782: f64, t81783: f64, t1516: f64, t81769: f64, t23133: f64, t4261: f64, t25111: f64, t25115: f64, t87229: f64, t23132: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87328 = t81782 * t81783 * t25097;
    let t87330 = t81769 * t1516;
    let t87332 = t23133 * t4261;
    let t87335 = t81782 * t81783 * t25111;
    let t87338 = t87229 * t81783 * t25115;
    let t87340 = t4166 * t23132;
    (t87328, t87330, t87332, t87335, t87338, t87340)
}
