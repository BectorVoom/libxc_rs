//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 612/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk612<F: Float>(t446: F, t9034: F, t8796: F, t8799: F, t8802: F, t8805: F, t9010: F, t9014: F, t9020: F, t9024: F, t9028: F, t9032: F, t2205: F, t7807: F, t1651: F, t558: F) -> (F, F, F, F, F) {
    let t9035 = t446 * t9034;
    let t9037 = -2.0 / 27.0 * t8796 + t8799 / 18.0 + t8802 / 27.0 - t8805 / 3.0 - t9010 / 6.0 - t9014 / 18.0 - t9020 + t9024 - 5.0 / 81.0 * t9028 - t9032 / 3.0 + t9035 / 3.0;
    let t9038 = t2205 * t7807;
    let t9039 = t446 * t9038;
    let t9041 = t1651 * t558;
    (t9035, t9037, t9038, t9039, t9041)
}
