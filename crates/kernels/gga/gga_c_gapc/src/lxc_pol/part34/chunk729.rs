//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 729/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk729<F: Float>(t116: F, t5312: F, t3708: F, t5407: F, t676: F, t8986: F, t5260: F, t178: F, t8700: F, t3109: F, t1404: F, t1720: F, t3108: F, t5553: F, t8687: F, t19: F, t8768: F) -> (F, F, F, F, F, F, F) {
    let t9113 = t116 * t5312;
    let t9114 = t3708 * t5407;
    let t9115 = t9113 * t9114;
    let t9117 = t8986 * t676;
    let t9118 = t5260 * t9117;
    let t9120 = t178 * t8700;
    let t9121 = t9120 * t3109;
    let t9123 = t1720 * t1404;
    let t9124 = t3108 * t9123;
    let t9126 = t5553 * t8687;
    let t9128 = t8768 * t19;
    (t9113, t9115, t9118, t9121, t9124, t9126, t9128)
}
