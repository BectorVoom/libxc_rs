//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 534/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk534<F: Float>(t2549: F, t3248: F, t7221: F, t883: F, t2562: F, t943: F, t3240: F, t2558: F, t2717: F, t2537: F, t2554: F, t7064: F) -> (F, F, F, F, F, F) {
    let t9622 = F::cast_from(0.64087718584518535698e-3_f64) * t2549 * t3248;
    let t9624 = t883 * t7221;
    let t9625 = t2562 * t9624;
    let t9627 = F::cast_from(0.64087718584518535698e-3_f64) * t943 * t9625;
    let t9629 = F::cast_from(0.64087718584518535698e-3_f64) * t2549 * t3240;
    let t9630 = t2717 * t2558;
    let t9632 = F::cast_from(0.64087718584518535698e-3_f64) * t943 * t9630;
    let t9633 = t2537 * t2554;
    let t9635 = F::cast_from(0.64087718584518535698e-3_f64) * t7064 * t9633;
    (t9622, t9624, t9627, t9629, t9632, t9635)
}
