//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1032/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1032<F: Float>(t43681: F, t43107: F, t701: F, t6066: F, t7630: F, t1457: F, t2103: F, t43316: F, t13024: F, t5771: F, t13016: F, t8638: F) -> (F, F, F, F, F, F) {
    let t43682 = F::cast_from(0.3575048995185042667e0_f64) * t43681;
    let t43683 = t43107 * t701;
    let t43686 = F::cast_from(0.71500979903700853338e0_f64) * t7630 * t6066 * t43683;
    let t43690 = t2103 * t1457 * t43316;
    let t43693 = F::cast_from(0.71500979903700853338e0_f64) * t5771 * t13024;
    let t43695 = F::cast_from(0.10725146985555128001e1_f64) * t8638 * t13016;
    (t43682, t43683, t43686, t43690, t43693, t43695)
}
