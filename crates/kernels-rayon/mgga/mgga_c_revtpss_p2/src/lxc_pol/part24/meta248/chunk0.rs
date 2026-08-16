//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1011/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1011(t136: f64, t1568: f64, t2457: f64, t2710: f64, t2470: f64, t4522: f64, t874: f64, t2718: f64, t1569: f64, t867: f64, t786: f64, t2435: f64, t4477: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14946 = t1568 * t136;
    let t14948 = t2710 * t14946 * t2457;
    let t14951 = t874 * t4522 * t2470;
    let t14961 = t2718 * t1568;
    let t14986 = t1569 * t867;
    let t14987 = t786 * t14986;
    let t14998 = t2435 * t4477;
    (t14946, t14948, t14951, t14961, t14986, t14987, t14998)
}
