//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 827/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk827<F: Float>(t29375: F, t7429: F, t17969: F, t9047: F, t17933: F, t9072: F, t28294: F, t5290: F, t5289: F, t28256: F, t5322: F, t28957: F, t41: F, t719: F, t734: F, t28324: F) -> (F, F, F, F, F, F, F) {
    let t29376 = t7429 * t29375;
    let t29378 = t17969 * t9047;
    let t29380 = t17933 * t9072;
    let t29382 = t5290 * t28294;
    let t29383 = t5289 * t29382;
    let t29385 = t5322 * t28256;
    let t29386 = t7429 * t29385;
    let t29388 = t28957 * t41;
    let t29389 = t29388 * t719;
    let t29390 = t734 * t29389;
    let t29392 = t5322 * t28324;
    (t29376, t29378, t29380, t29383, t29386, t29390, t29392)
}
