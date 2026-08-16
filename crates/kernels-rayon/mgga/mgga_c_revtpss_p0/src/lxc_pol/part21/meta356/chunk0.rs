//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1706/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1706(t11144: f64, t11821: f64, t10356: f64, t1012: f64, t11150: f64, t3252: f64, t11156: f64, t4919: f64, t11165: f64, t4915: f64, t1066: f64, t11169: f64, t247: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11822 = t11821 * t11144;
    let t11823 = t11822 * t10356;
    let t11824 = t1012 * t11823;
    let t11827 = t3252 * t11150;
    let t11828 = t11827 * t10356;
    let t11829 = t1012 * t11828;
    let t11836 = t4919 * t11156;
    let t11839 = t4915 * t11165;
    let t11845 = t247 * t1066 * t11169;
    (t11823, t11824, t11828, t11829, t11836, t11839, t11845)
}
