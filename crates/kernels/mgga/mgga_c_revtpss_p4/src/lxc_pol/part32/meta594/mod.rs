//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta594<F: Float>(t103363: F, t25299: F, t2439: F, t780: F, t785: F, t7997: F, t7407: F, t99272: F, t26482: F, t99404: F, t98849: F, t25305: F) -> (F, F, F, F, F, F) {
        let (t103364, t103370, t103382, t103391, t103393, t103394) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1926::<F>(t103363, t25299, t2439, t780, t785, t7997, t7407, t99272, t26482, t99404, t98849, t25305);
    (t103364, t103370, t103382, t103391, t103393, t103394)
}
