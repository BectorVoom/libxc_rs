//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1325/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1325<F: Float>(t28903: F, t8669: F, t20703: F, t20706: F, t20714: F, t24556: F, t24559: F, t24562: F, t28853: F, t28856: F, t28859: F, t796: F) -> (F, F, F) {
    let t28907 = t8669 * t28903;
    let t28916 = t20714 - F::new(56.0) / F::new(27.0) * t20703 + F::new(4.0) / F::new(9.0) * t20706 - F::new(56.0) / F::new(27.0) * t24556 + F::new(16.0) / F::new(9.0) * t24559 - F::new(2.0) / F::new(3.0) * t24562 + F::new(4.0) / F::new(9.0) * t28859 - F::new(2.0) / F::new(3.0) * t28853 + t28856;
    let t28917 = t796 * t28916;
    (t28907, t28916, t28917)
}
