//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1202/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1202(t1527: f64, t7537: f64, t2718: f64, t1911: f64, t5636: f64, t10110: f64, t5657: f64, t16815: f64, t232: f64, t6646: f64, t1888: f64, t5544: f64, t6638: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28306 = t7537 * t1527;
    let t28307 = t2718 * t28306;
    let t28310 = t1911 * t5636;
    let t28311 = t10110 * t28310;
    let t28316 = t1911 * t5657;
    let t28317 = t2718 * t28316;
    let t28321 = t16815 * t232;
    let t28322 = t6646 * t28321;
    let t28323 = t1888 * t28322;
    let t28329 = t6638 * t5544;
    (t28307, t28311, t28317, t28321, t28322, t28323, t28329)
}
