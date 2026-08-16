//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 903/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk903(t9285: f64, t9287: f64, t2365: f64, t6520: f64, t7025: f64, t1415: f64, t2371: f64, t7030: f64, t1645: f64, t2349: f64, t3196: f64, t7014: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9289 = 0.29792074959875355558e-1_f64 * t9285 * t9287;
    let t9294 = t2365 * t6520;
    let t9296 = 0.29792074959875355558e-1_f64 * t7025 * t9294;
    let t9305 = t1415 * t2371;
    let t9307 = 0.29792074959875355558e-1_f64 * t9305 * t7030;
    let t9333 = t1645 * t2349;
    let t9362 = t7014 * t3196;
    (t9289, t9294, t9296, t9305, t9307, t9333, t9362)
}
