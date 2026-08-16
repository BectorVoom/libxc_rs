//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1014/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1014<F: Float>(t30612: F, t30637: F, t1191: F, t1172: F, t13023: F, t30558: F, t13021: F, t13027: F, t19100: F, t25590: F, t25601: F, t25609: F, t30569: F, t30572: F, t30592: F, t30595: F, t30599: F, t30603: F) -> (F, F, F) {
    let t30638 = t30612 + t30637;
    let t30639 = t30638 * t1191;
    let t30641 = F::cast_from(1.0_f64) * t1172 * t30639;
    let t30642 = t30558 * t13023;
    let t30644 = F::cast_from(0.51725014705706168417e3_f64) * t13021 * t30642;
    let t30655 = -t13027 - F::cast_from(0.12361111111111111111e-1_f64) * t19100 + F::cast_from(0.61805555555555555556e-2_f64) * t25590 - F::cast_from(0.18541666666666666667e-1_f64) * t25601 + F::cast_from(0.92708333333333333334e-2_f64) * t25609 - F::cast_from(0.10300925925925925926e-1_f64) * t30592 + F::cast_from(0.37083333333333333333e-1_f64) * t30595 - F::cast_from(0.18541666666666666666e-1_f64) * t30569 - F::cast_from(0.55625000000000000001e-1_f64) * t30599 + F::cast_from(0.55625000000000000001e-1_f64) * t30572 - F::cast_from(0.92708333333333333333e-2_f64) * t30603;
    (t30641, t30644, t30655)
}
