//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 919/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk919<F: Float>(t43683: F, t6066: F, t7630: F, t13024: F, t5771: F, t13016: F, t8638: F, t1645: F, t3025: F, t9689: F, t23477: F, t42945: F, t4820: F) -> (F, F, F, F, F) {
    let t43686 = F::cast_from(0.71500979903700853338e0_f64) * t7630 * t6066 * t43683;
    let t43693 = F::cast_from(0.71500979903700853338e0_f64) * t5771 * t13024;
    let t43695 = F::cast_from(0.10725146985555128001e1_f64) * t8638 * t13016;
    let t43698 = F::cast_from(0.10725146985555128001e1_f64) * t3025 * t1645 * t9689;
    let t43708 = F::cast_from(0.23833659967900284446e0_f64) * t23477 * t4820 * t42945;
    (t43686, t43693, t43695, t43698, t43708)
}
