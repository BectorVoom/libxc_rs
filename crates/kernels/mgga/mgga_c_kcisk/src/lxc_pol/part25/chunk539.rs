//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 539/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk539<F: Float>(t1248: F, t4640: F, t4893: F, t1720: F, t4644: F, t4648: F, t4838: F, t4842: F, t4845: F, t4848: F, t4866: F, t4874: F, t4876: F, t4882: F, t4884: F, t4888: F, t4891: F) -> (F, F, F, F) {
    let t4895 = t1248 * t4893 * t4640;
    let t4898 = t1248 * t1720 * t4644;
    let t4901 = t1248 * t1720 * t4648;
    let t4903 = -0.9494625e0 * t4866 + 0.1898925e1 * t4874 + t4876 + 0.19931111111111111111e0 * t4838 - 0.19931111111111111111e0 * t4842 + 0.59793333333333333334e0 * t4845 - 0.29896666666666666667e0 * t4848 + 0.15358125e0 * t4882 + 0.3071625e0 * t4884 + t4888 + 0.21908444444444444444e0 * t4891 - 0.5477111111111111111e-1 * t4895 + 0.32862666666666666666e0 * t4898 - 0.16431333333333333333e0 * t4901;
    (t4895, t4898, t4901, t4903)
}
