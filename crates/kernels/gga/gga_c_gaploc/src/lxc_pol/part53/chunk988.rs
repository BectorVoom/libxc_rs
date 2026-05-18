//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 988/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk988<F: Float>(t1841: F, t2576: F, t39347: F, t13937: F, t731: F, t1897: F, t1901: F, t47322: F, t13921: F, t7137: F, t7129: F, t2508: F, t2580: F, t47220: F) -> (F, F, F, F, F, F) {
    let t47696 = t1841 * t39347 * t2576;
    let t47702 = t731 * t13937;
    let t47708 = F::new(0.76905262301422242837e-2) * t1897 * t1901 * t47322;
    let t47709 = t7137 * t13921;
    let t47711 = t7129 * t13921;
    let t47714 = t2508 * t2580 * t47220;
    (t47696, t47702, t47708, t47709, t47711, t47714)
}
