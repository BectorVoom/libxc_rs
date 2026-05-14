//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 699/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk699<F: Float>(t2386: F, t240: F, t1686: F, t1987: F, t2396: F, t5423: F, t6801: F, t6804: F, t6806: F, t6809: F, t6837: F, t6841: F, t6848: F, t6857: F, t6876: F, t6881: F, t7513: F) -> (F, F) {
    let t7517 = t240 * t2386;
    let t7528 = -t6801 + t6804 + t6806 - t6809 + t6837 + t6841 + t240 * t7513 + 0.19751789702565206229e-1 * t240 * t6848 - 0.58482233974552040708e0 * t7517 * t1686 - 0.58482233974552040708e0 * t5423 * t2396 + 0.11696446794910408142e1 * t1987 * t6857 - 0.58482233974552040708e0 * t1987 * t6876 - 0.17315755899375863299e2 * t1987 * t6881;
    (t7517, t7528)
}
