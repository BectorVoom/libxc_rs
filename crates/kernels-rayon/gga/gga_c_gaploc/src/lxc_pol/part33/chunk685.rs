//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 685/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk685(t1305: f64, t2334: f64, t1064: f64, t2293: f64, t599: f64, t475: f64, t2343: f64, t1595: f64, t876: f64, t1324: f64, t894: f64, t1265: f64, t2344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6424 = t2334 * t1305;
    let t6425 = t1064 * t6424;
    let t6428 = t599 * t2293;
    let t6429 = t6428 * t475;
    let t6430 = t2343 * t6429;
    let t6433 = t1595 * t876;
    let t6438 = t894 * t1324;
    let t6443 = t2344 * t1265;
    (t6424, t6425, t6428, t6429, t6430, t6433, t6438, t6443)
}
