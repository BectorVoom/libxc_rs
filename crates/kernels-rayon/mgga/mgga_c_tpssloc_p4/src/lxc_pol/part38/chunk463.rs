//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 463/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk463(t1539: f64, t882: f64, t123: f64, t881: f64, t291: f64, t880: f64) -> (f64, f64, f64, f64, f64) {
    let t1540 = t882 * t1539;
    let t1541 = t123 * t1540;
    let t1543 = -t881 - 0.17808333333333333333e-1_f64 * t1541;
    let t1545 = 0.621814e-1_f64 * t1543 * t291;
    let t1547 = -t880 / 3.0_f64 - t1541 / 3.0_f64;
    (t1540, t1541, t1543, t1545, t1547)
}
