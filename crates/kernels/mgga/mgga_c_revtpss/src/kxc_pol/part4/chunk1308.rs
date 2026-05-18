//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1308/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1308<F: Float>(t1082: F, t15648: F, t3291: F, t4757: F, t3059: F, t5004: F, t16426: F, t3318: F, t1043: F, t1089: F, t4930: F, t15717: F) -> (F, F, F, F, F, F) {
    let t16479 = t1082 * t15648;
    let t16482 = t3291 * t4757;
    let t16485 = t5004 * t3059;
    let t16488 = t16426 * t3318;
    let t16496 = t4930 * t1043 * t1089;
    let t16499 = t1082 * t15717;
    (t16479, t16482, t16485, t16488, t16496, t16499)
}
