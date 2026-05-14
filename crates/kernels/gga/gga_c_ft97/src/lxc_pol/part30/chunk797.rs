//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 797/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk797<F: Float>(t2862: F, t36240: F, t871: F, t319: F, t35833: F, t6353: F, t7045: F, t840: F, t296: F, t36101: F, t1091: F, t7686: F, t835: F, t7105: F, t36005: F, t1255: F, t7584: F) -> (F, F, F, F, F, F, F, F) {
    let t36242 = t2862 * t871 * t36240;
    let t36246 = t2862 * t319 * t35833;
    let t36250 = t840 * t6353 * t7045;
    let t36253 = t296 * t36101;
    let t36257 = t835 * t7686 * t1091;
    let t36261 = t840 * t6353 * t7105;
    let t36264 = t296 * t36005;
    let t36268 = t2862 * t1255 * t7584;
    (t36242, t36246, t36250, t36253, t36257, t36261, t36264, t36268)
}
