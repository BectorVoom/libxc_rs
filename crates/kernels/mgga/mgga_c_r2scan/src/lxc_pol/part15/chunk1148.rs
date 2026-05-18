//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1148/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1148<F: Float>(t39770: F, t37890: F, t924: F, t24031: F, t3332: F, t7628: F, t24035: F, t6165: F, t11646: F, t22731: F, t11649: F, t25169: F) -> (F, F, F, F, F, F) {
    let t39771 = F::new(0.25610080155860322884e0) * t39770;
    let t39772 = t37890 * t924;
    let t39775 = t7628 * t3332 * t24031;
    let t39778 = t6165 * t3332 * t24035;
    let t39780 = t22731 * t11646;
    let t39782 = t25169 * t11649;
    (t39771, t39772, t39775, t39778, t39780, t39782)
}
