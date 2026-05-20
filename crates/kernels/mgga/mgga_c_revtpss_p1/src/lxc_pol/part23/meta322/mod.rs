//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta322<F: Float>(t1711: F, t9617: F, t2: F, t3881: F, t1892: F, t785: F, t1358: F, t2439: F, t1903: F, t4075: F, t1444: F, t556: F) -> (F, F, F, F, F, F, F, F) {
        let (t13701, t13704, t13725, t13726, t13727, t13729, t13730, t13731) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1611::<F>(t1711, t9617, t2, t3881, t1892, t785, t1358, t2439, t1903, t4075, t1444, t556);
    (t13701, t13704, t13725, t13726, t13727, t13729, t13730, t13731)
}
