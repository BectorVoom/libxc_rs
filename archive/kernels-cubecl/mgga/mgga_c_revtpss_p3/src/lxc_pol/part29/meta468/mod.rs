//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1725;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1726;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1727;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1728;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta468<F: Float>(t25231: F, t25242: F, t25253: F, t25275: F, t25283: F, t25251: F, t25256: F, t25258: F, t25263: F, t25267: F, t25271: F, t25278: F, t25280: F, t25223: F, t25225: F, t25229: F, t25235: F, t25238: F, t25246: F, t25248: F, t26450: F, t233: F, t1957: F, t122: F, t2061: F, t72: F, t25412: F, t25411: F, t2466: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26454, t26457, t26462, t26468, t26471, t26472) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1725::<F>(t25231, t25242, t25253, t25275, t25283, t25251, t25256, t25258, t25263, t25267, t25271, t25278, t25280);
        let t26473 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1726::<F>(t25223, t25225, t25229, t25235, t25238, t25246, t25248, t26450, t26454, t26457, t26472);
        let (t26474, t26475, t26481) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1727::<F>(t233, t26473, t1957, t122, t2061, t72);
        let t26482 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1728::<F>(t25412, t26481);
        let (t26483, t26485) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1729::<F>(t25411, t26482, t2466, t26481);
    (t26454, t26457, t26462, t26468, t26471, t26473, t26474, t26475, t26481, t26482, t26483, t26485)
}
