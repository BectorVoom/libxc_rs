//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1011/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1011(t34004: f64, t572: f64, t7330: f64, t7741: f64, t1916: f64, t8614: f64, t1518: f64, t32374: f64, t1918: f64, t33992: f64, t33996: f64, t33998: f64, t34000: f64, t34003: f64, t573: f64, t8607: f64, t8616: f64) -> (f64, f64, f64, f64, f64) {
    let t34006 = 6.0_f64 * t572 * t34004;
    let t34007 = t7330 * t7741;
    let t34009 = 12.0_f64 * t572 * t34007;
    let t34010 = t1916 * t8614;
    let t34011 = 3.0_f64 * t34010;
    let t34012 = t32374 * t1518;
    let t34013 = t572 * t34012;
    let t34014 = 6.0_f64 * t34013;
    let t34015 = 3.0_f64 * t1918 * t8607 + t33992 * t573 + 6.0_f64 * t33996 + 12.0_f64 * t33998 + 6.0_f64 * t34000 + t34003 + t34006 + t34009 + t34011 + t34014 + t8616;
    (t34007, t34011, t34012, t34014, t34015)
}
