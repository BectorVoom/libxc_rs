//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1864;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta533<F: Float>(t26435: F, t9303: F, t26440: F, t686: F, t72: F, t25375: F, t2470: F, t26543: F, t7058: F, t122: F, t25412: F, t7398: F, t25431: F, t2646: F, t26481: F, t676: F, t26482: F, t93374: F, t7385: F, t9292: F, t2772: F, t689: F, t7384: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t95569, t95571, t95572, t95575, t95576, t95593) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1864::<F>(t26435, t9303, t26440, t686, t72, t25375, t2470, t26543, t7058, t122, t25412, t7398);
        let (t95594, t95597, t95598, t95604, t95607, t95613) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1865::<F>(t25431, t95593, t2646, t26481, t676, t26482, t93374, t7385, t9292, t2772, t689, t7384);
    (t95569, t95571, t95572, t95575, t95576, t95593, t95594, t95597, t95598, t95604, t95607, t95613)
}
