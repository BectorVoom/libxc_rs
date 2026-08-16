//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1034/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1034<F: Float>(t14620: F, t14625: F, t14628: F, t14630: F, t14865: F, t15481: F, t15482: F, t15485: F, t15486: F, t15487: F, t15491: F, t15495: F, t15499: F, t15894: F, t68336: F, t73660: F) -> F {
    let t79954 = -t14620 - t15481 - t15482 + t15894 + t15485 + t15486 - t15487 + t15491 + t14625 - t14628 + t68336 + t15495 + t15499 - t73660 + t14630 - t14865;
    t79954
}
