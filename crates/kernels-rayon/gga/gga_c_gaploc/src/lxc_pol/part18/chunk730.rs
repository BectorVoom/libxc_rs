//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 730/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk730(t587: f64, t6855: f64, t1323: f64, t487: f64, t2365: f64, t4361: f64, t600: f64, t6393: f64, t568: f64, t1508: f64, t894: f64, t1589: f64, t2349: f64) -> (f64, f64, f64, f64, f64) {
    let t6856 = t587 * t6855;
    let t6858 = t487 * t1323;
    let t6859 = t2365 * t6858;
    let t6860 = t4361 * t6859;
    let t6862 = t600 * t6393;
    let t6863 = t568 * t6862;
    let t6866 = t1508 * t894;
    let t6869 = t1589 * t2349;
    (t6856, t6860, t6863, t6866, t6869)
}
