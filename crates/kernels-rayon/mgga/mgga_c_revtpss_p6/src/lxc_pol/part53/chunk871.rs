//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 871/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk871(t2122: f64, t25163: f64, t2139: f64, t3655: f64, t1256: f64, t7610: f64, t2138: f64, t3666: f64, t3678: f64, t7613: f64, t3685: f64, t7607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26795 = t2122 * t25163;
    let t26821 = 0.95275595817932748827e-4_f64 * t2139 * t3655;
    let t26822 = t7610 * t1256;
    let t26827 = t3666 * t2138;
    let t26832 = t7613 * t3678;
    let t26836 = t7607 * t3685;
    (t26795, t26821, t26822, t26827, t26832, t26836)
}
