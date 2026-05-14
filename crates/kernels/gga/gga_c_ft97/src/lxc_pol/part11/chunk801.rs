//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 801/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk801<F: Float>(t11: F, t1690: F, t7868: F, t5544: F, t8018: F, t1685: F, t1597: F, t1663: F, t78: F, t8153: F, t8157: F, t32: F, t8991: F, t122: F, t31: F, t7911: F) -> (F, F, F, F, F, F, F) {
    let t38176 = t1690 * t11;
    let t38177 = t38176 * t7868;
    let t38180 = t5544 * t8018;
    let t38187 = t1685 * t1685;
    let t38192 = t1597 * t1663 * t78;
    let t38195 = t8153 * t8157;
    let t38200 = t8991 / t32;
    let t38211 = t122 / t31 / t7911;
    (t38177, t38180, t38187, t38192, t38195, t38200, t38211)
}
