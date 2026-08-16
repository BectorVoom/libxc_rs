//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 937/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk937(t76707: f64, t73807: f64, t73812: f64, t73817: f64, t73827: f64, t73833: f64, t73840: f64, t73843: f64, t73847: f64, t73849: f64, t76688: f64, t76689: f64, t76690: f64, t76693: f64, t76696: f64, t76701: f64, t76703: f64) -> f64 {
    let t76708 = 0.1064114997332445985e-4_f64 * t76707;
    let t76709 = -0.58171619854173713846e-5_f64 * t73807 - 0.58171619854173713846e-5_f64 * t73812 + t76688 + t73817 - t76689 + t76690 - 0.4379826523225341797e-6_f64 * t73827 - 0.1532939283128869629e-5_f64 * t73833 - t76693 - 0.8759653046450683594e-6_f64 * t73840 + 0.13139479569676025391e-5_f64 * t73843 - t76696 - 0.58171619854173713846e-5_f64 * t73847 - t73849 - t76701 - t76703 - t76708;
    t76709
}
