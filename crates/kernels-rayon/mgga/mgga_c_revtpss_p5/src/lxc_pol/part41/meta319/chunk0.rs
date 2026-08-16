//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1096/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1096(t2689: f64, t5618: f64, t1413: f64, t5591: f64, t547: f64, t807: f64, t5609: f64, t808: f64, t9845: f64, t1885: f64, t9909: f64, t1399: f64, t1872: f64, t9818: f64) -> (f64, f64, f64, f64, f64) {
    let t13949 = t2689 * t5618;
    let t13951 = t1413 * t5591;
    let t13952 = t547 * t13951;
    let t13954 = 0.57165357490759649296e-4_f64 * t807 * t13952;
    let t13955 = t808 * t5609;
    let t13956 = t9845 * t13955;
    let t13959 = t9909 * t1885;
    let t13985 = t9818 * t1872 * t1399;
    (t13949, t13954, t13956, t13959, t13985)
}
