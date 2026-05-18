//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 905/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk905<F: Float>(t1459: F, t8286: F, t475: F, t4855: F, t2953: F, t3652: F, t1603: F, t3639: F, t1006: F, t1005: F, t3946: F, t1577: F) -> (F, F, F, F, F, F, F, F) {
    let t11248 = t8286 * t1459;
    let t11249 = t475 * t4855;
    let t11250 = t11248 * t11249;
    let t11252 = t2953 * t3652;
    let t11254 = t3639 * t1603;
    let t11255 = t1006 * t11254;
    let t11257 = t1005 * t3946;
    let t11258 = t3639 * t1577;
    (t11248, t11249, t11250, t11252, t11254, t11255, t11257, t11258)
}
