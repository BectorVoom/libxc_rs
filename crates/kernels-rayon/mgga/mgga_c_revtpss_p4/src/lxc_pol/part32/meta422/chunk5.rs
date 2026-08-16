//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1490/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1490(t19456: f64, t996: f64, t1678: f64, t4746: f64, t1695: f64, t5015: f64, t3269: f64, t6343: f64, t994: f64, t19462: f64, t378: f64, t4772: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20188 = t996 * t19456;
    let t20191 = t4746 * t1678;
    let t20194 = t1695 * t5015;
    let t20195 = t3269 * t20194;
    let t20204 = t994 * t6343;
    let t20211 = t19462 * t378;
    let t20214 = t4772 * t1695;
    (t20188, t20191, t20195, t20204, t20211, t20214)
}
