//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 616/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk616(t2761: f64, t66: f64, t2460: f64, t242: f64, t2690: f64, t2693: f64, t2700: f64, t2706: f64, t2722: f64, t2727: f64, t2731: f64, t2734: f64, t2740: f64, t2743: f64, t2748: f64, t2754: f64, t2757: f64, t925: f64, t946: f64, t967: f64, t972: f64) -> (f64, f64) {
    let t2762 = t66 * t2761;
    let t2763 = t2762 * t2460;
    let t2764 = t242 * t2763;
    let t2767 = t2690 / 432.0_f64 + t925 * t2693 / 288.0_f64 + t925 * t2700 / 216.0_f64 + t946 * t2706 / 3072.0_f64 + t2722 * t2727 / 1536.0_f64 - t2731 * t2734 / 3072.0_f64 + t2740 * t2743 / 2304.0_f64 - t2748 * t972 / 432.0_f64 + t2754 / 3456.0_f64 + t967 * t2757 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t967 * t2764;
    (t2762, t2767)
}
