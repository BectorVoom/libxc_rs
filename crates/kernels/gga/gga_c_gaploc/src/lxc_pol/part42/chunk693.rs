//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 693/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk693<F: Float>(t13437: F, t1445: F, t1562: F, t3377: F, t3566: F, t11362: F, t12969: F, t13397: F, t912: F, t587: F, t6915: F, t6914: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13438 = t1445 * t13437;
    let t13440 = F::cast_from(0.69017266717057349418e1_f64) * t1562 * t13438;
    let t13442 = F::cast_from(0.25025342966295298669e1_f64) * t3566 * t3377;
    let t13444 = F::cast_from(0.10725146985555128001e1_f64) * t11362 * t3377;
    let t13463 = F::cast_from(0.17875244975925213335e0_f64) * t12969;
    let t13465 = t912 * t13397;
    let t13466 = t587 * t13465;
    let t13468 = t6915 * t13397;
    let t13469 = t6914 * t13468;
    (t13438, t13440, t13442, t13444, t13463, t13465, t13466, t13468, t13469)
}
