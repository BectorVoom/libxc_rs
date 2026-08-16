//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 618/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk618(t226: f64, t3664: f64, t773: f64, t774: f64, t3629: f64, t783: f64, t3628: f64, t125: f64, t1364: f64, t2175: f64, t1385: f64, t2383: f64) -> (f64, f64, f64, f64, f64) {
    let t3665 = t3664 * t226;
    let t3667 = t773 * t774 * t3665;
    let t3670 = t3629 * t783;
    let t3671 = t3628 * t3670;
    let t3676 = t125 * t1364;
    let t3677 = t3676 * t783;
    let t3678 = t2175 * t3677;
    let t3681 = t2383 * t1385;
    (t3665, t3667, t3671, t3678, t3681)
}
