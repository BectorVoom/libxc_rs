//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 584/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk584<F: Float>(t1670: F, t240: F, t1686: F, t1987: F, t4698: F, t4701: F, t4708: F, t4739: F, t4747: F, t4754: F, t4764: F, t4783: F, t4791: F, t5419: F, t2028: F, t791: F) -> (F, F, F, F) {
    let t5423 = t240 * t1670;
    let t5432 = -t4698 + t4701 - t4708 + t4739 + t4747 + t240 * t5419 + 0.19751789702565206229e-1 * t240 * t4754 - 0.11696446794910408142e1 * t5423 * t1686 + 0.11696446794910408142e1 * t1987 * t4764 - 0.58482233974552040708e0 * t1987 * t4783 - 0.17315755899375863299e2 * t1987 * t4791;
    let t5437 = t2028 * t2028;
    let t5438 = t791 * t791;
    (t5423, t5432, t5437, t5438)
}
