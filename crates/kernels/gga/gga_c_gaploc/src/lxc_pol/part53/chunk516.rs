//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 516/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk516<F: Float>(t9823: F, t9824: F, t1966: F, t1991: F, t2028: F, t9789: F, t9790: F, t9793: F, t9799: F, t9803: F, t9809: F, t9812: F, t9815: F, t9817: F, t9822: F, t165: F, t2530: F) -> (F, F, F) {
    let t9826 = 0.29792074959875355558e-1 * t9823 * t9824;
    let t9827 = t9789 - 0.25561950635947166451e1 * t1966 * t9790 + 0.51123901271894332902e0 * t1991 * t9793 - t9799 + t9803 - t9809 + t9812 + t9815 - 0.39722766613167140743e-1 * t9817 * t2028 - t9822 + t9826;
    let t9828 = t165 * t2530;
    (t9826, t9827, t9828)
}
