//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 915/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk915(t2006: f64, t213: f64, t225: f64, t22573: f64, t8449: f64, t2627: f64, t8543: f64, t23030: f64, t31381: f64, t22690: f64, t23171: f64, t31376: f64) -> (f64, f64, f64, f64, f64) {
    let t114285 = t213 * t2006 * t225;
    let t114360 = t8449 * t22573;
    let t114655 = t2627 * t8543;
    let t114672 = t23030 * t31381;
    let t114673 = 0.26044789391763585244e-1_f64 * t114672;
    let t114688 = t23171 * t22690 * t31376;
    (t114285, t114360, t114655, t114673, t114688)
}
