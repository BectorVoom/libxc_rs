//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1004/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1004(t114770: f64, t22986: f64, t28267: f64, t28276: f64, t31366: f64, t6552: f64, t23035: f64, t31376: f64, t5527: f64, t6637: f64, t121495: f64, t1510: f64, t6646: f64) -> (f64, f64, f64, f64) {
    let t127952 = t22986 * t114770 * t28267;
    let t127955 = t6552 * t31366 * t28276;
    let t127959 = t23035 * t6637 * t31376 * t5527;
    let t127963 = t22986 * t6646 * t121495 * t1510;
    (t127952, t127955, t127959, t127963)
}
