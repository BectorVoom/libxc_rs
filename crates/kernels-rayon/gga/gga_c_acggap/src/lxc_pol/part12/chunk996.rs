//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 996/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk996(t1619: f64, t322: f64, t157: f64, t524: f64, t929: f64, t30028: f64, t615: f64, t8790: f64, t33643: f64, t315: f64, t32123: f64, t309: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33699 = t1619 * t322;
    let t33706 = t524 * t929 * t157;
    let t33727 = t615 * t30028;
    let t33735 = t8790 * t929;
    let t33739 = t33643 * t157;
    let t33743 = t315 * t32123;
    let t33744 = t1619 * t309;
    (t33699, t33706, t33727, t33735, t33739, t33743, t33744)
}
