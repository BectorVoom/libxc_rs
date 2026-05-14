//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 232/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk232<F: Float>(t1039: F, t206: F, t190: F, t974: F, t214: F, t1001: F, t982: F, t212: F, t8: F) -> (F, F, F, F, F, F, F) {
    let t1040 = t206 * t1039;
    let t1042 = t974 * t190;
    let t1043 = t1042 * t214;
    let t1045 = t214 * t1001;
    let t1046 = t982 * t1045;
    let t1048 = t212 * t8;
    let t1049 = 1.0 / t1048;
    (t1040, t1042, t1043, t1045, t1046, t1048, t1049)
}
