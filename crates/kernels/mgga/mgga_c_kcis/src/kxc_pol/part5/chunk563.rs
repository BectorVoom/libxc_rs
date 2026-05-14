//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 563/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk563<F: Float>(t1157: F, t3393: F, t1160: F, t238: F, t86: F, t2840: F, t41: F, t2844: F, t339: F, t1083: F, t330: F, t1071: F, t381: F, t386: F) -> (F, F, F, F, F, F, F) {
    let t3394 = t3393 * t1157;
    let t3397 = t86 * t238 * t1160;
    let t3399 = t41 * t2840;
    let t3400 = t339 * t2844;
    let t3405 = t1083 * t330;
    let t3410 = t339 * t1071;
    let t3436 = t381 * t386;
    (t3394, t3397, t3399, t3400, t3405, t3410, t3436)
}
