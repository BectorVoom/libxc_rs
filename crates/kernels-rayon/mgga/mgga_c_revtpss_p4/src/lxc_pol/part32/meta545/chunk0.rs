//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1858/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1858(t2062: f64, t2769: f64, t786: f64, t26519: f64, t93157: f64, t2453: f64, t2458: f64, t7399: f64, t2070: f64, t41154: f64, t11064: f64, t7427: f64) -> (f64, f64, f64, f64, f64) {
    let t95936 = t786 * t2062 * t2769;
    let t95945 = t93157 * t26519;
    let t95948 = t2453 * t7399 * t2458;
    let t95964 = t2070 * t41154;
    let t95976 = t7427 * t11064;
    (t95936, t95945, t95948, t95964, t95976)
}
