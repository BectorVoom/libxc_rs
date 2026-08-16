//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 859/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk859(t1569: f64, t867: f64, t786: f64, t2435: f64, t4477: f64, t136: f64, t1579: f64, t2457: f64, t10504: f64, t2471: f64, t4325: f64, t1580: f64, t2440: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14986 = t1569 * t867;
    let t14987 = t786 * t14986;
    let t14998 = t2435 * t4477;
    let t15002 = t1579 * t136;
    let t15003 = t15002 * t2457;
    let t15004 = t10504 * t15003;
    let t15006 = t4325 * t2471;
    let t15014 = t2440 * t1580;
    (t14987, t14998, t15003, t15004, t15006, t15014)
}
