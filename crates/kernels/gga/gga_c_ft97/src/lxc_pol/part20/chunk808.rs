//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 808/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk808<F: Float>(t25026: F, t26: F, t1476: F, t2756: F, t852: F, t193: F, t24967: F, t24971: F, t24974: F, t24978: F, t24984: F, t24987: F, t24992: F, t24995: F, t24998: F, t25003: F, t25007: F, t25010: F, t25015: F, t25020: F, t25024: F) -> (F, F, F, F) {
    let t25027 = t25026 * t26;
    let t25028 = t1476 * t2756;
    let t25029 = t852 * t25028;
    let t25031 = t25027 * t193 * t25029;
    let t25033 = 4.0 * t24967 + 2.0 * t24971 - t24974 / 6.0 - 2.0 / 3.0 * t24978 - t24984 / 6.0 - 4.0 / 3.0 * t24987 - 6.0 * t24992 + 2.0 / 3.0 * t24995 - 4.0 / 3.0 * t24998 + t25003 + 2.0 * t25007 - 2.0 / 3.0 * t25010 + t25015 / 4.0 + t25020 / 2.0 - 3.0 * t25024 - 3.0 / 8.0 * t25031;
    (t25027, t25029, t25031, t25033)
}
