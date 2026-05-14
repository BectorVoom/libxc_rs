//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 728/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk728<F: Float>(t3827: F, t3844: F, t3847: F, t3849: F, t3851: F, t3853: F, t3855: F, t3857: F, t3859: F, t3861: F, t3863: F, t3865: F, t3867: F, t3869: F, t3871: F, t3873: F, t51: F, t565: F) -> (F,) {
    let t3875 = t51 * t3827 / 6.0 - t565 * t3844 / 18.0 - t3847 / 48.0 + t3849 / 240.0 + t3851 / 640.0 - t3853 / 4480.0 - t3855 / 11520.0 + t3857 / 103680.0 + t3859 / 258048.0 - t3861 / 2838528.0 - t3863 / 6881280.0 + t3865 / 89456640.0 + t3867 / 0.21233664e9 - t3869 / 0.31850496e10 - t3871 / 0.74317824e10 + t3873 / 0.1263403008e12;
    (t3875,)
}
