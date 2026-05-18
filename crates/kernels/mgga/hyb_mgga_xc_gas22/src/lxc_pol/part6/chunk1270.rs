//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1270/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1270<F: Float>(t3840: F, t6012: F, t3836: F, t3832: F, t1179: F, t1796: F, t1808: F, t1895: F, t1897: F, t1898: F, t1903: F, t19756: F, t23295: F, t23341: F, t23351: F, t3008: F, t3014: F, t3023: F, t3814: F, t545: F, t572: F, t575: F, t7945: F, t9872: F, t9877: F, t9888: F, t9909: F) -> F {
    let t27333 = t6012 * t3840;
    let t27335 = t6012 * t3836;
    let t27341 = t6012 * t3832;
    let t27348 = -t572 * t3014 * t9888 * t1796 / F::new(9.0) - F::new(5.0) / F::new(243.0) * t572 * t7945 * t9872 * t1796 - F::new(40.0) / F::new(729.0) * t572 * t23295 * t19756 * t3814 * t1808 + F::new(2.0) / F::new(27.0) * t572 * t3008 * t9877 * t1796 - F::new(142.0) / F::new(243.0) * t23341 + F::new(28.0) / F::new(729.0) * t23351 + F::new(4.0) / F::new(27.0) * t3023 * t575 * t1903 * t1179 - F::new(2.0) / F::new(243.0) * t27333 + F::new(4.0) / F::new(243.0) * t27335 - F::new(4.0) / F::new(81.0) * t3023 * t1895 * t1898 * t1179 - F::new(4.0) / F::new(729.0) * t27341 - F::new(2.0) / F::new(81.0) * t572 * t3008 * t1897 * t9909 * t545;
    t27348
}
