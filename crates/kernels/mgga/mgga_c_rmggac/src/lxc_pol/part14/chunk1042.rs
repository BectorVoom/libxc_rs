//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1042/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1042<F: Float>(t1971: F, t236: F, t5704: F, t7365: F, t35331: F, t5700: F, t36772: F, t9147: F, t615: F, t7230: F, t839: F, t880: F) -> (F, F, F, F) {
    let t41690 = t7365 * t1971 * t236 * t5704;
    let t41694 = t35331 * t1971 * t236 * t5700;
    let t41696 = t36772 * t9147;
    let t41701 = t7230 * t1971 * t880 * t615 * t839;
    (t41690, t41694, t41696, t41701)
}
