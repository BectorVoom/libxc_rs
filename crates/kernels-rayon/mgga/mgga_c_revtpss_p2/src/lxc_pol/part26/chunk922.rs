//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 922/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk922(t1026: f64, t371: f64, t676: f64, t1025: f64, t271: f64, t2857: f64, t11144: f64, t10356: f64, t1012: f64, t11150: f64, t3252: f64, t11156: f64, t4919: f64) -> (f64, f64, f64, f64) {
    let t11817 = t371 * t676 * t1026;
    let t11818 = t1025 * t11817;
    let t11821 = 1.0_f64 / t271 / t2857;
    let t11822 = t11821 * t11144;
    let t11823 = t11822 * t10356;
    let t11824 = t1012 * t11823;
    let t11827 = t3252 * t11150;
    let t11828 = t11827 * t10356;
    let t11829 = t1012 * t11828;
    let t11836 = t4919 * t11156;
    (t11818, t11824, t11829, t11836)
}
