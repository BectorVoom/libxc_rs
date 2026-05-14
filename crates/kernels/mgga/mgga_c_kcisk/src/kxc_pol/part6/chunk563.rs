//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 563/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk563<F: Float>(t524: F, t1550: F, t2107: F, t240: F, t6568: F, t7750: F, t7752: F, t7756: F, t7788: F, t7791: F, t7797: F, t7804: F, t7821: F, t7825: F, t8384: F, t1589: F, t1586: F) -> (F, F, F, F) {
    let t536 = 0.0 < t524;
    let t8396 = -t7750 + t7752 - t7756 + t7788 + t7791 + t240 * t8384 + 0.19751789702565206229e-1 * t240 * t7797 - 0.11696446794910408142e1 * t6568 * t2107 + 0.11696446794910408142e1 * t1550 * t7804 - 0.58482233974552040708e0 * t1550 * t7821 - 0.17315755899375863299e2 * t1550 * t7825;
    let t8398 = piecewise3(t536, t8396, -t8396);
    let t8399 = t1589 * t8398;
    let t8400 = t1586 * t8399;
    (t8396, t8398, t8399, t8400)
}
