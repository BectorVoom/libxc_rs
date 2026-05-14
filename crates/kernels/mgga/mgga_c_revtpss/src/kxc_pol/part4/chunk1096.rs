//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1096/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1096<F: Float>(t136: F, t1579: F, t2457: F, t10504: F, t2471: F, t4325: F, t1580: F, t2444: F, t689: F, t213: F, t4469: F, t2440: F, t2439: F, t1569: F, t2453: F, t2458: F) -> (F, F, F, F, F, F) {
    let t15002 = t1579 * t136;
    let t15003 = t15002 * t2457;
    let t15004 = t10504 * t15003;
    let t15006 = t4325 * t2471;
    let t15008 = t2444 * t1580;
    let t15010 = 0.10975748638225852664e-1 * t689 * t15008;
    let t15011 = t213 * t4469;
    let t15014 = t2440 * t1580;
    let t15015 = t2439 * t15014;
    let t15017 = t2453 * t1569;
    let t15018 = t15017 * t2458;
    (t15004, t15006, t15010, t15011, t15015, t15018)
}
