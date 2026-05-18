//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1154/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1154<F: Float>(t2306: F, t820: F, t9385: F, t3975: F, t3972: F, t13948: F, t13954: F, t13962: F, t13964: F, t14664: F, t14669: F, t14674: F, t14678: F, t14680: F, t14685: F, t14689: F, t14693: F, t3066: F) -> (F, F, F) {
    let t14696 = t2306 * t820;
    let t14697 = t9385 * t14696;
    let t14698 = t3975 * t14697;
    let t14699 = t3972 * t14698;
    let t14703 = t3066 * t14664 / F::new(48.0) + t3066 * t14669 / F::new(48.0) + t14674 / F::new(96.0) + t14678 / F::new(96.0) + t14680 / F::new(96.0) + t14685 / F::new(1536.0) - F::new(7.0) / F::new(288.0) * t14689 - t13948 - t14693 / F::new(3072.0) + F::new(7.0) / F::new(288.0) * t13954 + t14699 / F::new(768.0) + F::new(7.0) / F::new(288.0) * t13962 + F::new(7.0) / F::new(4608.0) * t13964;
    (t14696, t14698, t14703)
}
