//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1173/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1173(t670: f64, t7683: f64, t2163: f64, t2371: f64, t2127: f64, t2165: f64, t2372: f64, t25193: f64, t25196: f64, t25804: f64, t25838: f64, t25840: f64, t25842: f64, t25844: f64, t25846: f64, t25853: f64, t25858: f64, t25860: f64, t25863: f64, t25868: f64, t26091: f64, t27060: f64, t3813: f64, t4151: f64, t651: f64, t671: f64, t7586: f64) -> (f64, f64, f64) {
    let t27076 = t7683 * t670;
    let t27079 = t2163 * t2371;
    let t27088 = -t2127 * t3813 + t2165 * t4151 - 2.0_f64 * t2372 * t7586 - 4.0_f64 * t27060 * t671 - 4.0_f64 * t27076 * t651 - 2.0_f64 * t27079 * t651 + t25193 - t25196 - t25804 + t25838 - t25840 - t25842 - t25844 + t25846 - t25853 - t25858 - t25860 - t25863 + t25868 + t26091;
    (t27076, t27079, t27088)
}
