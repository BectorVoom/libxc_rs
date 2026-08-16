//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 821/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk821(t32944: f64, t32960: f64, t32977: f64, t32949: f64, t32954: f64, t32957: f64, t32965: f64, t32970: f64, t32974: f64, t32982: f64, t32986: f64, t32990: f64) -> (f64, f64, f64, f64) {
    let t33106 = t32944 / 18.0_f64;
    let t33110 = 2.0_f64 / 9.0_f64 * t32960;
    let t33114 = t32977 / 9.0_f64;
    let t33118 = t33106 + t32949 / 18.0_f64 + t32954 / 3.0_f64 - t32957 / 6.0_f64 - t33110 - 2.0_f64 / 9.0_f64 * t32965 - 2.0_f64 * t32970 + 4.0_f64 / 3.0_f64 * t32974 + t33114 + t32982 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t32986 - t32990 / 3.0_f64;
    (t33106, t33110, t33114, t33118)
}
