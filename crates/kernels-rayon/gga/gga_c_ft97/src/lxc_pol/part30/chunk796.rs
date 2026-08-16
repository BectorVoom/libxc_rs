//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 796/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk796(t33864: f64, t33992: f64, t312: f64, t1476: f64, t1506: f64, t2665: f64, t684: f64, t317: f64, t7611: f64, t7584: f64, t10248: f64, t7662: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33993 = t33864 + t33992;
    let t33994 = t33993 * t312;
    let t33996 = t1476 * t1506;
    let t33998 = t2665 * t33996 * t684;
    let t34001 = t7611 * t317;
    let t34002 = t34001 * t684;
    let t34003 = t2665 * t34002;
    let t34006 = t7584 * t317;
    let t34008 = t10248 * t34006 * t684;
    let t34012 = t7662 * t870;
    (t33993, t33994, t33996, t33998, t34001, t34003, t34006, t34008, t34012)
}
