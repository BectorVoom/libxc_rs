//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 382/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk382<F: Float>(t1393: F, t939: F, t946: F, t1386: F, t341: F, t238: F, t242: F, t1388: F, t944: F, t951: F) -> (F, F, F, F, F) {
    let t1394 = t939 * t1393;
    let t1397 = t946 * t1393;
    let t1399 = t341 * t1386;
    let t1401 = t238 * t242 * t1399;
    let t1403 = 0.1898925e1 * t1394 - t944 + 0.8969e0 * t1388 + 0.3071625e0 * t1397 - t951 + 0.24647e0 * t1401;
    (t1394, t1397, t1399, t1401, t1403)
}
