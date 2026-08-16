//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1214/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1214(t12679: f64, t18690: f64, t5757: f64, t5936: f64, t1830: f64, t2105: f64, t1163: f64, t1273: f64, t13235: f64, t1760: f64, t1800: f64, t1834: f64, t18547: f64, t18613: f64, t18628: f64, t18680: f64, t18687: f64, t2056: f64, t2062: f64, t2065: f64, t3396: f64, t3499: f64, t485: f64, t5706: f64, t5799: f64, t5801: f64, t5809: f64, t5816: f64, t5820: f64, t5905: f64, t5910: f64, t5939: f64, t626: f64) -> (f64, f64, f64, f64) {
    let t18691 = t18690 * t12679;
    let t18694 = t5936 * t5757;
    let t18697 = t1830 * t2105;
    let t18704 = -2.0_f64 * t1163 * t5799 + 2.0_f64 * t1273 * t5905 - 2.0_f64 * t13235 * t1800 + 6.0_f64 * t1760 * t18687 - 2.0_f64 * t1760 * t18694 - 2.0_f64 * t1830 * t2062 + t1834 * t3396 - 6.0_f64 * t18547 * t18691 - 2.0_f64 * t18613 * t626 - 2.0_f64 * t18628 * t626 - t18680 * t485 - 2.0_f64 * t18697 * t626 - 4.0_f64 * t2056 * t5816 - 4.0_f64 * t2056 * t5820 - 4.0_f64 * t2065 * t5801 - 4.0_f64 * t3499 * t5809 - 4.0_f64 * t3499 * t5816 + 6.0_f64 * t5706 * t5910 - 2.0_f64 * t5706 * t5939;
    (t18691, t18694, t18697, t18704)
}
