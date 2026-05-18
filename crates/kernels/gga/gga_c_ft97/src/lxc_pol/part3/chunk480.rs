//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 480/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk480<F: Float>(t147: F, t184: F, t3658: F, t21: F, t1078: F, t648: F, t1079: F, t363: F, t649: F, t920: F, t18: F, t1577: F, t1064: F, t1080: F, t2240: F, t3597: F, t3601: F, t5: F, t620: F, t623: F, t650: F) -> (F, F, F, F, F, F, F, F, F) {
    let t148 = F::new(10000000.0) <= t147;
    let t3659 = t3658 * t184;
    let t3660 = t3659 * t21;
    let t3663 = t1078 * t648;
    let t3664 = t184 * t21;
    let t3665 = t3663 * t3664;
    let t3668 = t1079 * t363;
    let t3674 = t649 * t920;
    let t3677 = t184 * t18;
    let t3678 = t3677 * t1577;
    let t3682 = piecewise3::<f64>(t148, F::new(0.0), t5 * t3597 * t21 / F::new(4.0) + t3601 * t650 / F::new(4.0) + t5 * t1064 * t363 / F::new(4.0) + t2240 * t1080 / F::new(4.0) + t623 * t3660 / F::new(4.0) + t623 * t3665 / F::new(4.0) + t623 * t3668 / F::new(4.0) + t5 * t620 * t920 / F::new(4.0) + t623 * t3674 / F::new(4.0) + t623 * t3678 / F::new(2.0));
    (t3659, t3660, t3663, t3664, t3665, t3668, t3674, t3678, t3682)
}
