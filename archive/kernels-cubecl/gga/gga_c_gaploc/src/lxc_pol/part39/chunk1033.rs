//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1033/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1033<F: Float>(t1645: F, t3025: F, t9689: F, t13020: F, t5771: F, t23477: F, t42945: F, t4820: F, t10667: F, t123: F, t883: F, t2684: F, t2685: F) -> (F, F, F, F, F) {
    let t43698 = F::cast_from(0.10725146985555128001e1_f64) * t3025 * t1645 * t9689;
    let t43699 = t5771 * t13020;
    let t43708 = F::cast_from(0.23833659967900284446e0_f64) * t23477 * t4820 * t42945;
    let t43710 = t10667 * t123 * t883;
    let t43712 = t2684 * t2685 * t43710;
    (t43698, t43699, t43708, t43710, t43712)
}
