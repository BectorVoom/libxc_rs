//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 786/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk786<F: Float>(t13048: F, t3641: F, t1191: F, t12911: F, t3677: F, t1192: F, t3671: F, t3639: F, t1190: F, t3679: F, t330: F, t3721: F, t12885: F, t3725: F, t12916: F, t12919: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12935: F, t12937: F, t12939: F, t12943: F, t12946: F, t12948: F, t12954: F) -> (F, F, F, F, F, F, F) {
    let t13050 = 6.0 * t13048 * t3641;
    let t13051 = t12911 * t1191;
    let t13053 = 6.0 * t3677 * t13051;
    let t13054 = t1192 * t3671;
    let t13056 = 6.0 * t3639 * t13054;
    let t13058 = t3671 * t3679 * t1190;
    let t13060 = 0.48245472966453314466e2 * t3677 * t13058;
    let t13064 = 1.0 / t3721 / t330;
    let t13066 = t13064 * t12885 * t3725;
    let t13083 = -0.3883875e1 * t12916 + 0.247573125e0 * t12919 - 0.33547222222222222222e0 * t12922 - 0.301925e0 * t12927 - 0.40256666666666666668e0 * t12929 + 0.30192500000000000001e0 * t12931 + 0.20128333333333333333e0 * t12933 - 0.27595e0 * t12935 + 0.16557e0 * t12937 + 0.5519e-1 * t12939 - 0.36793333333333333333e-1 * t12943 - 0.82785e-1 * t12946 - 0.60385000000000000001e0 * t12948 + 0.12077e1 * t12954;
    (t13050, t13053, t13056, t13060, t13064, t13066, t13083)
}
