//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 905/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk905<F: Float>(t33331: F, t33332: F, t45369: F, t1457: F, t44995: F, t6060: F, t13691: F, t15766: F, t13077: F, t8634: F, t11765: F, t2718: F) -> (F, F, F, F, F) {
    let t45372 = F::cast_from(0.13803453343411469884e3_f64) * t33331 * t33332 * t45369;
    let t45375 = F::cast_from(0.21450293971110256001e1_f64) * t6060 * t1457 * t44995;
    let t45377 = F::cast_from(0.21450293971110256001e1_f64) * t15766 * t13691;
    let t45379 = F::cast_from(0.71500979903700853338e0_f64) * t13077 * t8634;
    let t45381 = F::cast_from(0.35750489951850426669e0_f64) * t2718 * t11765;
    (t45372, t45375, t45377, t45379, t45381)
}
