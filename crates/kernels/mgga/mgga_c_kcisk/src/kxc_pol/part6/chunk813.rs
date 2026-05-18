//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 813/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk813<F: Float>(t5507: F, t9176: F, t2634: F, t7624: F, t4419: F, t9177: F, t782: F, t2041: F, t9258: F, t3748: F, t8090: F, t8259: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t25045 = t5507 * t9176;
    let t25128 = t2634 * t7624;
    let t25130 = t4419 * t9177;
    let t25131 = t782 * t25130;
    let t25153 = t9258 * t2041;
    let t25306 = t3748 * t8090;
    let t25308 = t8259 * sigma0;
    (t25045, t25128, t25131, t25153, t25306, t25308)
}
