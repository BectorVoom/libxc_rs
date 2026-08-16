//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1029/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1029(t2670: f64, t7033: f64, t2482: f64, t27: f64, t7043: f64, t1941: f64, t243: f64, t2712: f64, t64: f64, t2710: f64, t826: f64, t7036: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25231 = t7033 * t2670;
    let t25234 = t2482 * t7043 * t27;
    let t25237 = t1941 * t243;
    let t25240 = t64 * t2712;
    let t25242 = t2710 * t25240 * t826;
    let t25245 = t2482 * t7036 * t27;
    (t25231, t25234, t25237, t25240, t25242, t25245)
}
