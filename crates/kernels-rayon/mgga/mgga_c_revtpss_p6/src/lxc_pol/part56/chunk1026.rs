//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1026/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1026(t198: f64, t206: f64, t8493: f64, t41154: f64, t2411: f64, t31858: f64, t8489: f64, t31844: f64, t8478: f64, t8479: f64, t246: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119706 = t198 * t206 * t8493;
    let t119711 = t8493 * t41154;
    let t119737 = t31858 * t2411;
    let t119747 = t198 * t206 * t8489;
    let t119751 = t8478 * t8479 * t31844;
    let t119752 = t826 * t246;
    (t119706, t119711, t119737, t119747, t119751, t119752)
}
