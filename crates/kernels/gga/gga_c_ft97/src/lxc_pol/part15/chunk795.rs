//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 795/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk795<F: Float>(t2: F, t43833: F, t295: F, t41751: F, t665: F, t7640: F, t2344: F, t2680: F, t309: F, t43537: F, t2360: F, t2842: F, t192: F, t33828: F, t870: F, t9570: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t43834 = t43833 * t2;
    let t43852 = t41751 * t295;
    let t43912 = t665 * t7640;
    let t43913 = t43912 * t2;
    let t43917 = t2344 * t2680;
    let t43918 = t43917 * t2;
    let t44042 = t43912 * t309;
    let t44121 = 280.0 / 81.0 * t43537;
    let t44204 = t2842 * t2360;
    let t44245 = t43917 * t309;
    let t44280 = t192 * t33828;
    let t44335 = t43833 * t309;
    let t44340 = t870 * t9570;
    (t43834, t43852, t43913, t43918, t44042, t44121, t44204, t44245, t44280, t44335, t44340)
}
