//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1103/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1103(t1426: f64, t1893: f64, t786: f64, t3917: f64, t136: f64, t1903: f64, t2457: f64, t9674: f64, t10175: f64, t5722: f64, t122: f64, t5721: f64) -> (f64, f64, f64, f64, f64) {
    let t14099 = t1893 * t1426;
    let t14100 = t786 * t14099;
    let t14102 = 0.19514881078765566038e-1_f64 * t14100 * t3917;
    let t14103 = t1903 * t136;
    let t14104 = t14103 * t2457;
    let t14105 = t9674 * t14104;
    let t14108 = 0.19514881078765566038e-1_f64 * t10175 * t5722;
    let t14109 = t5721 * t122;
    (t14100, t14102, t14105, t14108, t14109)
}
