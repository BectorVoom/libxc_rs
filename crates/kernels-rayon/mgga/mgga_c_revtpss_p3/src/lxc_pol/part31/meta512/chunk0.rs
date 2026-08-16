//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1852/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1852(t4924: f64, t7111: f64, t1058: f64, t7801: f64, t1659: f64, t7125: f64, t1972: f64, t4797: f64, t4845: f64, t7117: f64, t4857: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27460 = t7111 * t4924;
    let t27462 = t7801 * t1058;
    let t27464 = t1659 * t7125;
    let t27467 = t4797 * t1972;
    let t27471 = t7117 * t4845;
    let t27479 = t4857 * t1972;
    (t27460, t27462, t27464, t27467, t27471, t27479)
}
