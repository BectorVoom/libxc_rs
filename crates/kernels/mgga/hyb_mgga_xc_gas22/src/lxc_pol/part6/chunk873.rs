//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 873/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk873<F: Float>(t1046: F, t2713: F, t1047: F, t2729: F, t1101: F, t2751: F, t1067: F, t2813: F, t462: F, t483: F, t7237: F, t7238: F, t7241: F) -> (F, F, F, F, F, F, F, F) {
    let t7307 = t2713 * t1046;
    let t7308 = t7307 * t1047;
    let t7310 = F::cast_from(6.0_f64) * t2729 * t7308;
    let t7312 = F::cast_from(60.0_f64) * t2751 * t1101;
    let t7313 = t1067 * t2813;
    let t7314 = t462 * t7313;
    let t7316 = t483 * t7237;
    let t7317 = t7238 * t7241;
    (t7307, t7308, t7310, t7312, t7313, t7314, t7316, t7317)
}
