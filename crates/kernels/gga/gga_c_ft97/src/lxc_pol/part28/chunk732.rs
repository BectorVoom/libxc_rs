//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 732/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk732<F: Float>(t32979: F, t363: F, t1969: F, t446: F, t32709: F, t558: F, t28: F, t89: F, t32869: F, t526: F, t27: F, t32945: F, t32949: F, t32954: F, t32957: F, t32961: F, t32965: F, t32970: F, t32974: F, t32978: F) -> (F, F, F, F, F, F, F) {
    let t32980 = t32979 * t363;
    let t32981 = t1969 * t32980;
    let t32982 = t446 * t32981;
    let t32984 = t32709 * t558;
    let t32985 = t28 * t32984;
    let t32986 = t89 * t32985;
    let t32988 = t526 * t32869;
    let t32990 = t89 * t27 * t32988;
    let t32991 = t32945 + t32949 / 6.0 + t32954 - t32957 / 2.0 - t32961 - 2.0 / 3.0 * t32965 - 6.0 * t32970 + 4.0 * t32974 + t32978 + t32982 / 3.0 + 2.0 * t32986 - t32990;
    (t32981, t32982, t32984, t32986, t32988, t32990, t32991)
}
