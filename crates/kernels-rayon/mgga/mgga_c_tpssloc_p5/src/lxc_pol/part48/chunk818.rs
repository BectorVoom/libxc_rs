//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 818/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk818(t192: f64, t531: f64, t1982: f64, t1887: f64, t23056: f64, t253: f64, t254: f64) -> (f64, f64, f64) {
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t25038 = t23056 * t1887;
    let t25168 = t253 * t254;
    (t24995, t25038, t25168)
}
