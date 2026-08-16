//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3506/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3506(t15689: f64, t19985: f64, t53405: f64, t1065: f64, t372: f64, t6305: f64, t1011: f64, t1012: f64, t1045: f64, t11632: f64, t12131: f64, t15130: f64, t15135: f64, t15691: f64, t15700: f64, t15701: f64, t16228: f64, t19776: f64, t19980: f64, t19993: f64, t3253: f64, t4915: f64, t53728: f64, t53741: f64, t53881: f64, t53898: f64, t53901: f64, t53923: f64, t60717: f64, t63236: f64, t66062: f64, t66067: f64) -> (f64, f64) {
    let t66176 = t15689 * t53405 * t19985;
    let t66187 = t372 * t1065 * t6305;
    let t66204 = 0.20325460441158986416e-2_f64 * t53881 + t1011 * t4915 * t63236 / 48.0_f64 + 0.47637797908966374413e-3_f64 * t15700 * t19980 * t1045 * t15135 - 0.28582678745379824648e-3_f64 * t53898 - 0.1270341277572436651e-3_f64 * t53901 - 0.3811023832717309953e-3_f64 * t66176 + 0.17149607247227894789e-2_f64 * t15700 * t53728 * t66062 - 0.11433071498151929859e-2_f64 * t15700 * t15701 * t66067 + 0.60976381323476959249e-2_f64 * t53923 * t19993 + 0.17149607247227894789e-2_f64 * t53741 * t66187 * t11632 * t16228 + t1011 * t1012 * t3253 * t60717 / 108.0_f64 + 0.95275595817932748826e-3_f64 * t15700 * t19980 * t1045 * t15130 - 0.57165357490759649296e-3_f64 * t15689 * t15691 * t12131 * t19776;
    (t66187, t66204)
}
