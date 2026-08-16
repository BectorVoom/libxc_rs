//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 983/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk983<F: Float>(t1457: F, t2103: F, t43316: F, t13024: F, t5771: F, t13016: F, t8638: F, t1645: F, t3025: F, t9689: F, t13020: F, t1: F, t106: F, t13096: F, t316: F) -> (F, F, F, F, F, F) {
    let t43690 = t2103 * t1457 * t43316;
    let t43693 = F::cast_from(0.71500979903700853338e0_f64) * t5771 * t13024;
    let t43695 = F::cast_from(0.10725146985555128001e1_f64) * t8638 * t13016;
    let t43698 = F::cast_from(0.10725146985555128001e1_f64) * t3025 * t1645 * t9689;
    let t43699 = t5771 * t13020;
    let t43703 = t13096 * t1 * t106 * t316;
    (t43690, t43693, t43695, t43698, t43699, t43703)
}
