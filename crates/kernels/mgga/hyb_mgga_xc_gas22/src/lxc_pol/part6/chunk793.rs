//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 793/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk793<F: Float>(t1117: F, t1134: F, t1145: F, t1158: F, t1172: F, t1539: F, t2821: F, t2829: F, t2834: F, t2838: F, t2847: F, t2862: F, t2868: F, t2875: F, t2881: F, t3760: F, t4491: F, t4494: F, t4502: F, t4505: F, t4513: F, t4521: F, t4525: F, t4530: F) -> (F,) {
    let t4534 = -16.0 / 9.0 * t2821 * t4491 + 16.0 / 9.0 * t2829 * t4494 + 16.0 / 3.0 * t2838 * t4494 - 16.0 / 3.0 * t2834 * t4491 - 32.0 / 81.0 * t2862 * t4502 - 16.0 / 27.0 * t1172 * t4505 - 32.0 / 81.0 * t2847 * t4502 - 16.0 / 27.0 * t1158 * t4505 + 44.0 / 27.0 * t1172 * t4513 + 44.0 / 27.0 * t1158 * t4513 - 72.0 * t1134 * t3760 * t1539 - 8.0 * t1117 * t4521 + 21.0 * t2875 * t4525 + 3.0 * t2881 * t4525 + 15.0 * t2868 * t1145 * t4530;
    (t4534,)
}
