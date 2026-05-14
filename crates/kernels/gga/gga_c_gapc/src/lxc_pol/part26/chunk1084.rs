//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1084/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1084<F: Float>(t11261: F, t35517: F, t4868: F, t11235: F, t13537: F, t13541: F, t1577: F, t8286: F, t3646: F, t8489: F, t1464: F, t3651: F, t4059: F, t11248: F, t1444: F, t4855: F) -> (F, F, F, F, F, F) {
    let t35524 = t11261 * t35517 * t4868;
    let t35527 = t11261 * t11235 * t13537;
    let t35531 = t8286 * t13541 * t11235 * t1577;
    let t35533 = t8489 * t3646;
    let t35536 = t3651 * t4059 * t1464;
    let t35539 = t11248 * t1444 * t4855;
    (t35524, t35527, t35531, t35533, t35536, t35539)
}
