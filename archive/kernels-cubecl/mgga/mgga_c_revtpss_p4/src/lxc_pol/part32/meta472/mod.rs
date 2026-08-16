//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1701;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta472<F: Float>(t25253: F, t25275: F, t25283: F, t122: F, t2061: F, t72: F, t25412: F, t25411: F, t2466: F, t25387: F, t2062: F, t867: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26462, t26468, t26471, t26481, t26482, t26483, t26485, t26486, t26496) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1701::<F>(t25253, t25275, t25283, t122, t2061, t72, t25412, t25411, t2466, t25387, t2062, t867);
        let t26497 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1702::<F>(t26496, t786);
    (t26462, t26468, t26471, t26481, t26482, t26483, t26485, t26486, t26496, t26497)
}
