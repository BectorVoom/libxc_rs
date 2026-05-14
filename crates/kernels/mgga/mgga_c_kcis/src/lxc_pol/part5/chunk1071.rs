//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1071/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1071<F: Float>(t14785: F, t5099: F, t15085: F, t19588: F, t5180: F, t19614: F, t3338: F, t5046: F, t19789: F, t5047: F, t1130: F, t19655: F, t376: F, t375: F, t19619: F, t5176: F) -> (F, F, F, F, F, F) {
    let t19945 = t14785 * t5099;
    let t19947 = t15085 * t19588;
    let t19948 = t5180 * t19947;
    let t19950 = t3338 * t19614;
    let t19951 = t5046 * t19950;
    let t19953 = t5047 * t19789;
    let t19954 = t5046 * t19953;
    let t19956 = t1130 * t19655;
    let t19957 = t376 * t19956;
    let t19958 = t375 * t19957;
    let t19960 = t5176 * t19619;
    (t19945, t19948, t19951, t19954, t19958, t19960)
}
