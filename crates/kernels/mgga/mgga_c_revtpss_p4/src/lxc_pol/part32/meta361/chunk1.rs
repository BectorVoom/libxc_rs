//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1307/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1307<F: Float>(t11044: F, t4481: F, t2435: F, t4477: F, t136: F, t1579: F, t2457: F, t10504: F, t2471: F, t4325: F, t1580: F, t2444: F) -> (F, F, F, F, F, F) {
    let t14995 = F::cast_from(0.19514881078765566038e-1_f64) * t11044 * t4481;
    let t14998 = t2435 * t4477;
    let t15002 = t1579 * t136;
    let t15003 = t15002 * t2457;
    let t15004 = t10504 * t15003;
    let t15006 = t4325 * t2471;
    let t15008 = t2444 * t1580;
    (t14995, t14998, t15003, t15004, t15006, t15008)
}
