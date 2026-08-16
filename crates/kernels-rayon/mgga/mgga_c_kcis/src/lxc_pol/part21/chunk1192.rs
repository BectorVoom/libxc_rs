//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1192/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1192(t783: f64, t91827: f64, t91861: f64, t2538: f64, t26651: f64, t826: f64, t2153: f64, t35630: f64, t26416: f64, t8522: f64, t2626: f64, t26516: f64) -> (f64, f64, f64, f64, f64) {
    let t91863 = t783 * (t91827 + t91861);
    let t91866 = 6.0_f64 * t2538 * t26651 * t826;
    let t91869 = t35630 * t2153;
    let t91872 = 6.0_f64 * t8522 * t26416;
    let t91874 = 3.0_f64 * t26516 * t2626;
    (t91863, t91866, t91869, t91872, t91874)
}
