//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1871;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta539<F: Float>(t2453: F, t26496: F, t10506: F, t10510: F, t26497: F, t10073: F, t25402: F, t7056: F, t7398: F, t26481: F, t93182: F, t25411: F, t2754: F, t676: F, t136: F, t2457: F, t7423: F, t25299: F, t25431: F, t26555: F, t40270: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t95773, t95774, t95779, t95783, t95785, t95786) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1871::<F>(t2453, t26496, t10506, t10510, t26497, t10073, t25402, t7056, t7398, t26481, t93182, t25411);
        let (t95790, t95793, t95794, t95796, t95798, t95807) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1872::<F>(t26481, t2754, t676, t25411, t136, t2457, t7423, t25299, t25431, t95785, t26555, t40270);
    (t95773, t95774, t95779, t95783, t95786, t95790, t95793, t95794, t95796, t95798, t95807)
}
