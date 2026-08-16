//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1660/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1660(t10845: f64, t4430: f64, t1558: f64, t853: f64, t2749: f64, t2662: f64, t2661: f64, t4352: f64, t837: f64, t4416: f64, t221: f64, t2485: f64, t4424: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14716 = t10845 * t4430;
    let t14718 = t853 * t1558;
    let t14719 = t14718 * t2749;
    let t14720 = t2662 * t14719;
    let t14722 = 0.57165357490759649296e-4_f64 * t2661 * t14720;
    let t14723 = t4352 * t837;
    let t14724 = t2662 * t14723;
    let t14726 = 0.14291339372689912324e-4_f64 * t2661 * t14724;
    let t14727 = t4416 * t837;
    let t14728 = t2662 * t14727;
    let t14730 = 0.57165357490759649296e-4_f64 * t2661 * t14728;
    let t14732 = t2485 * t221 * t4424;
    (t14716, t14718, t14720, t14722, t14724, t14726, t14728, t14730, t14732)
}
