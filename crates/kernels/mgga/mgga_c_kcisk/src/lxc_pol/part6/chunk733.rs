//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 733/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk733<F: Float>(t2009: F, t9184: F, t4998: F, t9213: F, t2013: F, t2630: F, t7624: F, t4419: F, t9227: F, t782: F, t5507: F, t9176: F, t2634: F, t9177: F, t2041: F, t9258: F) -> (F, F, F, F, F, F, F, F) {
    let t24980 = t9184 * t2009;
    let t25006 = t4998 * t9213;
    let t25007 = t2013 * t25006;
    let t25024 = t2630 * t7624;
    let t25026 = t4419 * t9227;
    let t25027 = t782 * t25026;
    let t25045 = t5507 * t9176;
    let t25128 = t2634 * t7624;
    let t25130 = t4419 * t9177;
    let t25131 = t782 * t25130;
    let t25153 = t9258 * t2041;
    (t24980, t25007, t25024, t25027, t25045, t25128, t25131, t25153)
}
