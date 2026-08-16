//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 879/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk879(t4354: f64, t9775: f64, t10722: f64, t1565: f64, t136: f64, t1568: f64, t2457: f64, t2710: f64, t2470: f64, t4522: f64, t874: f64, t2718: f64) -> (f64, f64, f64, f64, f64) {
    let t14850 = t9775 * t4354;
    let t14866 = t10722 * t1565;
    let t14946 = t1568 * t136;
    let t14948 = t2710 * t14946 * t2457;
    let t14951 = t874 * t4522 * t2470;
    let t14961 = t2718 * t1568;
    (t14850, t14866, t14948, t14951, t14961)
}
