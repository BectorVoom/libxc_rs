//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3506/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3506<F: Float>(t15689: F, t19985: F, t53405: F, t1065: F, t372: F, t6305: F, t1011: F, t1012: F, t1045: F, t11632: F, t12131: F, t15130: F, t15135: F, t15691: F, t15700: F, t15701: F, t16228: F, t19776: F, t19980: F, t19993: F, t3253: F, t4915: F, t53728: F, t53741: F, t53881: F, t53898: F, t53901: F, t53923: F, t60717: F, t63236: F, t66062: F, t66067: F) -> (F, F) {
    let t66176 = t15689 * t53405 * t19985;
    let t66187 = t372 * t1065 * t6305;
    let t66204 = F::cast_from(0.20325460441158986416e-2_f64) * t53881 + t1011 * t4915 * t63236 / F::new(48.0) + F::cast_from(0.47637797908966374413e-3_f64) * t15700 * t19980 * t1045 * t15135 - F::cast_from(0.28582678745379824648e-3_f64) * t53898 - F::cast_from(0.1270341277572436651e-3_f64) * t53901 - F::cast_from(0.3811023832717309953e-3_f64) * t66176 + F::cast_from(0.17149607247227894789e-2_f64) * t15700 * t53728 * t66062 - F::cast_from(0.11433071498151929859e-2_f64) * t15700 * t15701 * t66067 + F::cast_from(0.60976381323476959249e-2_f64) * t53923 * t19993 + F::cast_from(0.17149607247227894789e-2_f64) * t53741 * t66187 * t11632 * t16228 + t1011 * t1012 * t3253 * t60717 / F::new(108.0) + F::cast_from(0.95275595817932748826e-3_f64) * t15700 * t19980 * t1045 * t15130 - F::cast_from(0.57165357490759649296e-3_f64) * t15689 * t15691 * t12131 * t19776;
    (t66187, t66204)
}
