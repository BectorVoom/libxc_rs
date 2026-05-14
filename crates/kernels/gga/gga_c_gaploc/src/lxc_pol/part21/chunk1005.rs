//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1005/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1005<F: Float>(t2013: F, t9851: F, t9847: F, t2464: F, t2465: F, t7221: F, t825: F, t22672: F, t2684: F, t3295: F, t10017: F, t2615: F, t10014: F, t7416: F, t1423: F, t2563: F) -> (F, F, F, F, F, F, F) {
    let t28864 = 0.17041300423964777634e0 * t2013 * t9851;
    let t28865 = t2013 * t9847;
    let t28873 = 0.17041300423964777634e0 * t825 * t2464 * t2465 * t7221;
    let t28876 = 0.11928910296775344344e1 * t2684 * t22672 * t3295;
    let t28878 = t2615 * t2464 * t10017;
    let t28880 = t7416 * t10014;
    let t28889 = t1423 * t2563;
    (t28864, t28865, t28873, t28876, t28878, t28880, t28889)
}
