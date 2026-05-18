//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 706/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk706<F: Float>(t225: F, t7910: F, t1892: F, t1955: F, t1903: F, t2022: F, t7296: F, t1882: F, t543: F, t7301: F, t545: F, t2028: F) -> (F, F, F, F, F, F, F, F) {
    let t7911 = t7910 * t225;
    let t7917 = t1955 * t1892;
    let t7920 = t2022 * t1903;
    let t7921 = t7296 * t7920;
    let t7925 = t2022 * t1882 * t543;
    let t7926 = t7301 * t7925;
    let t7929 = t545 * t7910;
    let t7930 = t2028 * t7929;
    (t7911, t7917, t7920, t7921, t7925, t7926, t7929, t7930)
}
