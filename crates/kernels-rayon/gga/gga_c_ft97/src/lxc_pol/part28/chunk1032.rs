//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1032/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1032(t1307: f64, t25846: f64, t1317: f64, t1800: f64, t28: f64, t3103: f64, t7165: f64, t32076: f64, t7238: f64, t7239: f64, t5617: f64, t6454: f64) -> (f64, f64, f64, f64, f64) {
    let t144986 = t1307 * t25846;
    let t144989 = t1317 * t28 * t1800 * t144986;
    let t144991 = t7165 * t3103;
    let t144994 = t7238 * t7239 * t32076 * t144991;
    let t144998 = t5617 * t6454;
    (t144986, t144989, t144991, t144994, t144998)
}
