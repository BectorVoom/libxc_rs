//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 556/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk556(t1317: f64, t376: f64, t5680: f64, t1307: f64, t1557: f64, t1882: f64, t5693: f64, t358: f64, t5617: f64, t8345: f64, t91: f64, t5665: f64, t5667: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22980 = t1317 * t376 * t5680;
    let t22986 = t1307 * t1557;
    let t22991 = t1882 * t5693;
    let t22993 = t5617 * t358;
    let t23008 = t91 * t8345;
    let t23016 = t5665 * t376 * t5667;
    (t22980, t22986, t22991, t22993, t23008, t23016)
}
