//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta907 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2914;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta907<F: Float>(t11294: F, t23565: F, t19128: F, t4590: F, t52219: F, t6145: F, t23467: F, t41883: F, t23547: F, t2869: F, t11385: F, t15396: F, t6141: F, t934: F, t23492: F, t698: F, t23471: F, t141: F, t77501: F, t930: F, t18987: F, t4606: F, t15118: F, t6120: F, t18950: F, t4614: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t77639, t77641, t77643, t77645, t77647, t77657) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2914::<F>(t11294, t23565, t19128, t4590, t52219, t6145, t23467, t41883, t23547, t2869, t11385, t15396, t6141, t934);
        let (t77663, t77667, t77670, t77672, t77674, t77676) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2915::<F>(t23492, t698, t23471, t141, t77501, t930, t18987, t4606, t15118, t6120, t18950, t4614);
    (t77639, t77641, t77643, t77645, t77647, t77657, t77663, t77667, t77670, t77672, t77674, t77676)
}
