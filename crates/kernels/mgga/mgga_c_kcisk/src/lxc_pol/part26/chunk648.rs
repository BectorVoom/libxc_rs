//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 648/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk648<F: Float>(t2097: F, t240: F, t1213: F, t1550: F, t2107: F, t4486: F, t5714: F, t5717: F, t5719: F, t5722: F, t5751: F, t5755: F, t5762: F, t5771: F, t5790: F, t5795: F, t6564: F) -> (F, F) {
    let t6568 = t240 * t2097;
    let t6579 = -t5714 + t5717 + t5719 - t5722 + t5751 + t5755 + t240 * t6564 + 0.19751789702565206229e-1 * t240 * t5762 - 0.58482233974552040708e0 * t6568 * t1213 - 0.58482233974552040708e0 * t4486 * t2107 + 0.11696446794910408142e1 * t1550 * t5771 - 0.58482233974552040708e0 * t1550 * t5790 - 0.17315755899375863299e2 * t1550 * t5795;
    (t6568, t6579)
}
