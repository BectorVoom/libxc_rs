//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1031/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1031<F: Float>(t122: F, t8832: F, t649: F, t1764: F, t1769: F, t8825: F, t17169: F, t17248: F, t17290: F, t1778: F, t1787: F, t2460: F, t2466: F, t4989: F, t664: F, t7219: F, t7239: F, t7243: F, t7264: F, t7270: F, t8807: F) -> (F, F) {
    let t23768 = t8832 * t122;
    let t23769 = t649 * t23768;
    let t23776 = t1764 * t8832;
    let t23779 = t8825 * t1769;
    let t23789 = 0.95950873152945691804e-1 * t17248 * t7239 + 0.19190174630589138361e0 * t17248 * t7243 - 0.52772980234120130494e0 * t23769 * t1787 + 0.28785261945883707542e0 * t17169 * t2466 + 0.28785261945883707542e0 * t7219 * t7270 + 0.52772980234120130494e0 * t23776 * t664 - 0.95950873152945691807e-1 * t23779 + 0.35981577432354634426e-1 * t17290 * t2460 + 0.17590993411373376831e0 * t23769 * t1778 - 0.57570523891767415084e0 * t7219 * t7264 + 0.17990788716177317213e-1 * t4989 * t8807;
    (t23768, t23789)
}
