//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1839/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1839(t4021: f64, t94497: f64, t2482: f64, t25981: f64, t27: f64, t550: f64, t7021: f64, t25273: f64, t540: f64, t1372: f64, t2019: f64, t9951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94498 = t94497 * t4021;
    let t94508 = t2482 * t25981 * t27;
    let t94513 = t7021 * t550;
    let t94519 = t25273 * t540;
    let t94520 = t94519 * t1372;
    let t94522 = t2019 * t9951;
    (t94498, t94508, t94513, t94519, t94520, t94522)
}
