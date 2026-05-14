//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1183/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1183<F: Float>(t1045: F, t3322: F, t2198: F, t3330: F, t1327: F, t6918: F, t238: F, t800: F, t8977: F, t8981: F, t8985: F, t2224: F, t3361: F, t3365: F, t1336: F, t6812: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26329 = 8.0 * t3322 * t1045;
    let t26341 = t3330 * t2198;
    let t26344 = t1327 * t6918;
    let t26385 = t238 * t800 * t8977;
    let t26389 = t238 * t800 * t8981;
    let t26392 = t238 * t800 * t8985;
    let t26411 = t238 * t2224 * t3361;
    let t26414 = t238 * t2224 * t3365;
    let t26417 = t238 * t6812 * t1336;
    (t26329, t26341, t26344, t26385, t26389, t26392, t26411, t26414, t26417)
}
