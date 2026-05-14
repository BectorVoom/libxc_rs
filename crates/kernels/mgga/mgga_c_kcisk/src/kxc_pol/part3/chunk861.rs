//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 861/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk861<F: Float>(t1440: F, t3283: F, t3797: F, t3796: F, t3482: F, t394: F, t4143: F, t13777: F, t1340: F, t1411: F, t1284: F, t1299: F, t3777: F, t3504: F, t3739: F, t3509: F) -> (F, F, F, F, F) {
    let t14203 = t3283 * t1440;
    let t14204 = t3797 * t14203;
    let t14205 = t3796 * t14204;
    let t14206 = t3482 * t14205;
    let t14208 = t394 * t4143;
    let t14209 = t14208 * t13777;
    let t14210 = t1340 * t14209;
    let t14211 = t1411 * t14210;
    let t14213 = t1299 * t1284;
    let t14214 = t14213 * t3777;
    let t14215 = t1340 * t14214;
    let t14216 = t1411 * t14215;
    let t14218 = t3739 * t3504;
    let t14220 = t3739 * t3509;
    (t14206, t14211, t14216, t14218, t14220)
}
