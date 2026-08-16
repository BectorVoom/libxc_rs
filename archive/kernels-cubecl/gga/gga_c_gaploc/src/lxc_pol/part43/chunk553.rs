//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 553/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk553<F: Float>(t9823: F, t9824: F, t1966: F, t1991: F, t2028: F, t9789: F, t9790: F, t9793: F, t9799: F, t9803: F, t9809: F, t9812: F, t9815: F, t9817: F, t9822: F) -> (F, F) {
    let t9826 = F::cast_from(0.29792074959875355558e-1_f64) * t9823 * t9824;
    let t9827 = t9789 - F::cast_from(0.25561950635947166451e1_f64) * t1966 * t9790 + F::cast_from(0.51123901271894332902e0_f64) * t1991 * t9793 - t9799 + t9803 - t9809 + t9812 + t9815 - F::cast_from(0.39722766613167140743e-1_f64) * t9817 * t2028 - t9822 + t9826;
    (t9826, t9827)
}
