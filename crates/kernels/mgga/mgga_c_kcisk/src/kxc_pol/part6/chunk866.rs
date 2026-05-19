//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 866/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk866<F: Float>(t10699: F, t28471: F, t10690: F, t28341: F, t4790: F, t10639: F, t15989: F, t16389: F, t22564: F, t22575: F, t22583: F, t22698: F, t22705: F, t22707: F, t28362: F, t28379: F, t28387: F, t28394: F, t28404: F) -> (F, F, F) {
    let t28472 = t28471 * t10699;
    let t28475 = t10690 * t28341;
    let t28476 = t28475 * t4790;
    let t28492 = -F::cast_from(0.40256666666666666668e0_f64) * t15989 + F::cast_from(0.247573125e0_f64) * t28362 + F::new(0.258925e1) * t28394 - t10639 - F::new(0.27595e0) * t16389 + F::new(0.5519e-1) * t22698 + F::cast_from(0.20128333333333333333e0_f64) * t22564 - F::cast_from(0.60385000000000000001e0_f64) * t22575 + F::cast_from(0.30192500000000000001e0_f64) * t22583 - F::new(0.33114e0) * t22705 + F::new(0.16557e0) * t22707 + F::new(0.49671e0) * t28404 - F::cast_from(0.60384999999999999999e0_f64) * t28379 + F::new(0.181155e1) * t28387;
    (t28472, t28476, t28492)
}
