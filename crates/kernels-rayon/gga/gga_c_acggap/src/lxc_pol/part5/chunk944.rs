//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 944/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk944(t1372: f64, t228: f64, t1357: f64, t2604: f64, t1381: f64, t2632: f64, t2627: f64, t922: f64, t96: f64, t2614: f64, t3992: f64, t1378: f64, t4: f64, t657: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14837 = 32.0_f64 * t1372 * t228;
    let t14852 = t1357 * t2604;
    let t14854 = t1381 * t2632;
    let t14856 = t1381 * t2627;
    let t14866 = t96 * t922;
    let t14880 = t3992 * t2614;
    let t14883 = t1378 * t4 * t657;
    (t14837, t14852, t14854, t14856, t14866, t14880, t14883)
}
