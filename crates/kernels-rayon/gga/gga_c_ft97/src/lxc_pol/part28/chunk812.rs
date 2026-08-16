//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 812/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk812(t32979: f64, t363: f64, t1969: f64, t446: f64, t32709: f64, t558: f64, t28: f64, t89: f64, t32869: f64, t526: f64, t27: f64, t32945: f64, t32949: f64, t32954: f64, t32957: f64, t32961: f64, t32965: f64, t32970: f64, t32974: f64, t32978: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32980 = t32979 * t363;
    let t32981 = t1969 * t32980;
    let t32982 = t446 * t32981;
    let t32984 = t32709 * t558;
    let t32985 = t28 * t32984;
    let t32986 = t89 * t32985;
    let t32988 = t526 * t32869;
    let t32990 = t89 * t27 * t32988;
    let t32991 = t32945 + t32949 / 6.0_f64 + t32954 - t32957 / 2.0_f64 - t32961 - 2.0_f64 / 3.0_f64 * t32965 - 6.0_f64 * t32970 + 4.0_f64 * t32974 + t32978 + t32982 / 3.0_f64 + 2.0_f64 * t32986 - t32990;
    (t32981, t32982, t32984, t32986, t32988, t32990, t32991)
}
