//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1111/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1111<F: Float>(t25042: F, t4050: F, t15260: F, t3948: F, t11228: F, t25756: F, t11210: F, t21054: F, t25076: F, t8286: F, t11208: F, t19916: F, t11185: F, t2953: F, t11249: F, t8352: F) -> (F, F, F, F, F, F, F) {
    let t35541 = t25042 * t4050;
    let t35543 = t35541 * t3948 * t15260;
    let t35545 = t11228 * t25756;
    let t35552 = t8286 * t25076 * t11210 * t21054;
    let t35555 = t11208 * t11210 * t19916;
    let t35557 = t2953 * t11185;
    let t35559 = t8352 * t11249;
    (t35541, t35543, t35545, t35552, t35555, t35557, t35559)
}
