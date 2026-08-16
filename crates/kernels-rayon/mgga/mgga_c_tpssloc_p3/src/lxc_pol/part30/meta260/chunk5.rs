//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1187/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1187(t265: f64, t394: f64, t202: f64, t6665: f64, t1877: f64, t1915: f64, t193: f64, t2522: f64, t6670: f64, t776: f64, t868: f64, t870: f64, t1068: f64, t1070: f64, t336: f64, t4700: f64, t6818: f64, t6822: f64) -> (f64, f64) {
    let t395 = t265 < t394;
    let t6829 = t202 * t6665;
    let t6834 = -t1877 * t6670 * t868 + 3.0_f64 * t1915 * t2522 * t776 + t193 * t6829 * t870;
    let t6835 = piecewise3(t395, t1070 * t193 * t336 * t6818 - t1068 * t4700 * t6822, t6834);
    (t6834, t6835)
}
