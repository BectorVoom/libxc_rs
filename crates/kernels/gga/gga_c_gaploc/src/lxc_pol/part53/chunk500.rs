//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 500/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk500<F: Float>(t943: F, t9625: F, t2549: F, t3240: F, t2558: F, t2717: F, t2537: F, t2554: F, t7064: F, t296: F, t3209: F) -> (F, F, F, F, F) {
    let t9627 = 0.64087718584518535698e-3 * t943 * t9625;
    let t9629 = 0.64087718584518535698e-3 * t2549 * t3240;
    let t9630 = t2717 * t2558;
    let t9632 = 0.64087718584518535698e-3 * t943 * t9630;
    let t9633 = t2537 * t2554;
    let t9635 = 0.64087718584518535698e-3 * t7064 * t9633;
    let t9636 = t296 * t3209;
    (t9627, t9629, t9632, t9635, t9636)
}
