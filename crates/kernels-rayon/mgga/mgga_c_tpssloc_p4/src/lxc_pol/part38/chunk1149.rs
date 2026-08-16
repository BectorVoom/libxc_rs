//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1149/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1149(t1569: f64, t2880: f64, t2862: f64, t4437: f64, t2888: f64, t4433: f64, t931: f64, t10813: f64, t1568: f64, t4472: f64, t950: f64, t1581: f64, t2924: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14429 = t1569 * t2880;
    let t14432 = t4437 * t2862;
    let t14435 = t4433 * t2888;
    let t14436 = t14435 * t931;
    let t14439 = t4437 * t2880;
    let t14442 = t1568 * t10813;
    let t14443 = t14442 * t2862;
    let t14450 = t4472 * t950;
    let t14453 = t1581 * t2924;
    (t14429, t14432, t14436, t14439, t14443, t14450, t14453)
}
