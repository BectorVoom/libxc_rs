//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1005/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1005<F: Float>(t3764: F, t7736: F, t1340: F, t3759: F, t19047: F, t25342: F, t3484: F, t19046: F, t25413: F, t5634: F, t19086: F, t1163: F, t8255: F, t3482: F, t19055: F, t6226: F) -> (F, F, F, F, F, F, F, F) {
    let t26946 = t3764 * t7736;
    let t26947 = t1340 * t26946;
    let t26948 = t3759 * t26947;
    let t26950 = t19047 * t25342;
    let t26951 = t3484 * t26950;
    let t26952 = t19046 * t26951;
    let t26954 = t5634 * t25413;
    let t26955 = t3484 * t26954;
    let t26956 = t19086 * t26955;
    let t26958 = t8255 * t1163;
    let t26959 = t3484 * t26958;
    let t26960 = t3482 * t26959;
    let t26962 = t19055 * t6226;
    (t26948, t26950, t26952, t26954, t26956, t26958, t26960, t26962)
}
