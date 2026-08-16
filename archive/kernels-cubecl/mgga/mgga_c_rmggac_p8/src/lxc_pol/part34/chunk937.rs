//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 937/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk937<F: Float>(t76707: F, t73807: F, t73812: F, t73817: F, t73827: F, t73833: F, t73840: F, t73843: F, t73847: F, t73849: F, t76688: F, t76689: F, t76690: F, t76693: F, t76696: F, t76701: F, t76703: F) -> F {
    let t76708 = F::cast_from(0.1064114997332445985e-4_f64) * t76707;
    let t76709 = -F::cast_from(0.58171619854173713846e-5_f64) * t73807 - F::cast_from(0.58171619854173713846e-5_f64) * t73812 + t76688 + t73817 - t76689 + t76690 - F::cast_from(0.4379826523225341797e-6_f64) * t73827 - F::cast_from(0.1532939283128869629e-5_f64) * t73833 - t76693 - F::cast_from(0.8759653046450683594e-6_f64) * t73840 + F::cast_from(0.13139479569676025391e-5_f64) * t73843 - t76696 - F::cast_from(0.58171619854173713846e-5_f64) * t73847 - t73849 - t76701 - t76703 - t76708;
    t76709
}
