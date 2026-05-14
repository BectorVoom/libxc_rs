//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 741/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk741<F: Float>(t3877: F, t3894: F, t3897: F, t3899: F, t3901: F, t3903: F, t3905: F, t3907: F, t3909: F, t3911: F, t3913: F, t3915: F, t3917: F, t3919: F, t3921: F, t3923: F, t51: F, t564: F) -> (F,) {
    let t3925 = t51 * t3877 / 6.0 - t564 * t3894 / 18.0 - t3897 / 48.0 + t3899 / 240.0 + t3901 / 640.0 - t3903 / 4480.0 - t3905 / 11520.0 + t3907 / 103680.0 + t3909 / 258048.0 - t3911 / 2838528.0 - t3913 / 6881280.0 + t3915 / 89456640.0 + t3917 / 0.21233664e9 - t3919 / 0.31850496e10 - t3921 / 0.74317824e10 + t3923 / 0.1263403008e12;
    (t3925,)
}
