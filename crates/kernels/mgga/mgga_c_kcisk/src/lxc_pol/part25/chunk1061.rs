//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1061/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1061<F: Float>(t17771: F, t17774: F, t17777: F, t17778: F, t17779: F, t17781: F, t17783: F, t17786: F, t17789: F, t17792: F, t18176: F, t18842: F, t18933: F, t240: F, t15772: F, t2059: F) -> (F, F) {
    let t18936 = t17771 - t17774 + t17777 - t17778 - t17779 + t17781 - t17783 - t17786 + t17789 + t17792 - t18176 + t240 * (t18842 + t18933);
    let t22192 = t15772 * t2059;
    (t18936, t22192)
}
