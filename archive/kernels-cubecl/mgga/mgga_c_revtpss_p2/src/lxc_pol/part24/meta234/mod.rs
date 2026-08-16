//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk993;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta234<F: Float>(t13665: F, t2630: F, t1857: F, t3860: F, t3863: F, t1892: F, t785: F, t1358: F, t2439: F, t1903: F, t4075: F, t5622: F, t9765: F) -> (F, F, F, F, F, F, F, F) {
        let (t13666, t13668, t13670, t13725, t13726, t13727, t13729, t13765) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk993::<F>(t13665, t2630, t1857, t3860, t3863, t1892, t785, t1358, t2439, t1903, t4075, t5622, t9765);
    (t13666, t13668, t13670, t13725, t13726, t13727, t13729, t13765)
}
