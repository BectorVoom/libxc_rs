//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1158/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1158<F: Float>(t14138: F, t14733: F, t1173: F, t3202: F, t13973: F, t14706: F, t14708: F, t14711: F, t14714: F, t14716: F, t14718: F, t14722: F, t14727: F, t14729: F, t14731: F, t3207: F) -> F {
    let t14734 = t14733 * t14138;
    let t14737 = t1173 * t3202;
    let t14739 = t14706 / F::new(768.0) - F::new(7.0) / F::new(288.0) * t14708 - t3207 * t14711 / F::new(16.0) - t14714 / F::new(48.0) - F::new(7.0) / F::new(2304.0) * t14716 + F::new(7.0) / F::new(288.0) * t14718 - t14722 / F::new(3072.0) + t14727 / F::new(3072.0) + t14729 / F::new(48.0) + t14731 / F::new(16.0) - t14734 / F::new(96.0) + F::new(7.0) / F::new(4608.0) * t13973 + t14737 / F::new(96.0);
    t14739
}
