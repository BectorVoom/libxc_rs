//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1117/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1117(t11094: f64, t1958: f64, t2752: f64, t28: f64, t111: f64, t2022: f64, t192: f64, t531: f64, t1982: f64, t7450: f64, t1914: f64, t193: f64, t200: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23742 = t1958 * t11094;
    let t23788 = t2752 * t28;
    let t23880 = t2022 * t111;
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t24999 = t7450 * t111;
    let t25013 = t193 * t200 * t1914;
    (t23742, t23788, t23880, t24995, t24999, t25013)
}
