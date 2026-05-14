//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 971/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk971<F: Float>(t2520: F, t9400: F, t1404: F, t7366: F, t9274: F, t7192: F, t7195: F, t7221: F, t9271: F, t9292: F, t2469: F, t3531: F, t9341: F, t9344: F, t7294: F, t7297: F, t7300: F, t7384: F, t7391: F, t9335: F, t9338: F, t9348: F) -> (F, F, F, F, F, F, F, F) {
    let t9402 = 0.16081979498692535067e2 * t9400 * t2520;
    let t9404 = 1.0 * t7366 * t1404;
    let t9409 = 0.34246666666666666666e-1 * t9274;
    let t9411 = -t7221 + 0.45662222222222222222e-1 * t7192 - 0.17123333333333333333e-1 * t7195 + 0.22831111111111111111e-1 * t9271 - t9409 + 0.5137e-1 * t9292;
    let t9415 = 2.0 * t2469 * t3531;
    let t9423 = 0.32862666666666666666e0 * t9341;
    let t9424 = 0.32862666666666666666e0 * t9344;
    let t9426 = 0.79724444444444444446e0 * t7192 - 0.29896666666666666667e0 * t7195 - t7391 + 0.54771111111111111111e0 * t7294 - 0.16431333333333333333e0 * t7297 - 0.16431333333333333333e0 * t7300 - t7384 + 0.142419375e1 * t9335 - 0.76790625e-1 * t9338 - t9423 - t9424 + 0.24647e0 * t9348;
    (t9402, t9404, t9409, t9411, t9415, t9423, t9424, t9426)
}
