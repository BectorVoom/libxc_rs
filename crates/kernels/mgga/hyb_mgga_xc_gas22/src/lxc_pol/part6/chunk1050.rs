//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1050/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1050<F: Float>(t575: F, t578: F, t9909: F, t3023: F, t572: F, t6010: F, t6013: F, t7933: F, t7936: F, t7938: F, t7943: F, t9870: F, t9874: F, t9879: F, t9883: F, t9886: F, t9890: F, t9894: F, t9897: F, t9901: F, t9906: F) -> (F, F) {
    let t9911 = t575 * t578 * t9909;
    let t9914 = -t6010 - F::new(2.0) / F::new(243.0) * t6013 - F::new(4.0) / F::new(243.0) * t7933 + t7936 - t7938 + F::new(2.0) / F::new(81.0) * t7943 + t9870 / F::new(243.0) - F::new(5.0) / F::new(243.0) * t572 * t9874 + F::new(2.0) / F::new(27.0) * t572 * t9879 - F::new(4.0) / F::new(81.0) * t3023 * t9883 - t9886 / F::new(81.0) - t572 * t9890 / F::new(9.0) + F::new(4.0) / F::new(27.0) * t3023 * t9894 + t9897 / F::new(162.0) - t572 * t9901 / F::new(81.0) + t572 * t9906 / F::new(27.0) - t572 * t9911 / F::new(54.0);
    (t9911, t9914)
}
