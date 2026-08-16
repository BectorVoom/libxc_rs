//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 549/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk549(t346: f64, t939: f64, t348: f64, t356: f64, t329: f64) -> (f64, f64, f64, f64) {
    let t2715 = 1.0_f64 / t939 / t346;
    let t2716 = t2715 * t348;
    let t2717 = t356 * t356;
    let t2719 = 1.0_f64 / t2717 / t329;
    (t2715, t2716, t2717, t2719)
}
