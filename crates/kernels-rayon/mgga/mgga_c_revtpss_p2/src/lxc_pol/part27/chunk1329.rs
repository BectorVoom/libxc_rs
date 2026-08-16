//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1329/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1329(t10415: f64, t1310: f64, t13207: f64, t13216: f64, t2127: f64, t2163: f64, t2320: f64, t26800: f64, t3813: f64, t508: f64, t7584: f64, t7586: f64, t7683: f64, t95017: f64, t95020: f64, t95023: f64, t95025: f64, t95032: f64, t95036: f64, t95038: f64, t95040: f64, t95042: f64, t95046: f64, t95049: f64, t95056: f64, t95058: f64, t96834: f64) -> f64 {
    let t97550 = -t10415 * t2163 - 3.0_f64 * t1310 * t26800 - t13207 * t2127 - 6.0_f64 * t13216 * t7586 - 3.0_f64 * t2320 * t7683 - 3.0_f64 * t3813 * t7584 - t508 * t96834 - t95017 + t95020 + t95023 + t95025 - t95032 + t95036 - t95038 - t95040 - t95042 + t95046 - t95049 + t95056 + t95058;
    t97550
}
