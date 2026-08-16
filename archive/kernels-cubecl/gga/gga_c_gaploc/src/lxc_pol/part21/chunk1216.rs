//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1216/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1216<F: Float>(t32584: F, t25462: F, t2558: F, t9647: F, t25055: F, t5539: F, t16880: F, t25059: F, t1854: F, t22008: F, t32348: F, t21488: F, t314: F, t320: F, t3487: F, t7291: F, t734: F) -> (F, F, F, F, F, F) {
    let t32585 = F::cast_from(0.64087718584518535698e-3_f64) * t32584;
    let t32587 = t9647 * t25462 * t2558;
    let t32588 = F::cast_from(0.64087718584518535698e-3_f64) * t32587;
    let t32590 = t9647 * t5539 * t25055;
    let t32591 = F::cast_from(0.38452631150711121418e-2_f64) * t32590;
    let t32593 = t9647 * t16880 * t25059;
    let t32594 = F::cast_from(0.19226315575355560709e-2_f64) * t32593;
    let t32597 = F::cast_from(0.17090058289204942853e-2_f64) * t22008 * t32348 * t1854;
    let t32604 = F::cast_from(0.17090058289204942853e-2_f64) * t21488 * t320 * t314 * t7291 * t3487 * t734;
    (t32585, t32588, t32591, t32594, t32597, t32604)
}
