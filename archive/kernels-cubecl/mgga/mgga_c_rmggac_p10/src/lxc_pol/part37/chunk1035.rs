//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1035/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1035<F: Float>(t14638: F, t14641: F, t14649: F, t14971: F, t14973: F, t14974: F, t14975: F, t15240: F, t15243: F, t15246: F, t15501: F, t15506: F, t70720: F, t70721: F, t70722: F) -> F {
    let t79958 = -t14971 - t15501 + t15506 - t14638 - t14641 + t70720 + t14973 - t14974 + t14975 + t15240 - t15243 - t15246 - t70721 + t70722 + t14649;
    t79958
}
