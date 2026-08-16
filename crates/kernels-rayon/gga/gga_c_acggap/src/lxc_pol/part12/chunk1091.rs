//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1091/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1091(t2030: f64, t35413: f64, t4263: f64, t2299: f64, t7780: f64, t3196: f64, t33953: f64, t13364: f64, t31195: f64, t7637: f64, t8545: f64, t1429: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35415 = t2030 * t35413 * t4263;
    let t35418 = t7780 * t2299;
    let t35420 = t33953 * t3196;
    let t35422 = t31195 * t13364 * t35420;
    let t35425 = t7637 * t8545;
    let t35436 = t7614 * t1429;
    (t35415, t35418, t35420, t35422, t35425, t35436)
}
