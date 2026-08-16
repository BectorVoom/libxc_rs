//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 947/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk947(t1619: f64, t322: f64, t33698: f64, t620: f64, t1614: f64, t7927: f64, t2138: f64, t2147: f64, t463: f64, t8418: f64, t315: f64, t32123: f64) -> (f64, f64, f64, f64) {
    let t33699 = t1619 * t322;
    let t33702 = 0.10408353825846239354e2_f64 * t33698 * t620 * t33699;
    let t33715 = t7927 * t1614;
    let t33726 = 0.34694512752820797848e1_f64 * t2138 * t2147 * t8418 * t463;
    let t33743 = t315 * t32123;
    (t33702, t33715, t33726, t33743)
}
