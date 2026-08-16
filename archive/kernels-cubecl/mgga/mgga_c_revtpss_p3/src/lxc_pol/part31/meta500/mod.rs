//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1817;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta500<F: Float>(t25304: F, t7283: F, t25946: F, t25949: F, t786: F, t7286: F, t1426: F, t3999: F, t213: F, t7274: F, t116: F, t7002: F, t13426: F, t1937: F, t18227: F, t4248: F, t6993: F, t7003: F, t1518: F, t648: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26069, t26071, t26072, t26073, t26079, t26084, t26123) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1817::<F>(t25304, t7283, t25946, t25949, t786, t7286, t1426, t3999, t213, t7274, t116, t7002);
        let (t27116, t27118, t27120, t27122, t27123) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1818::<F>(t13426, t1937, t18227, t4248, t6993, t7003, t1518, t648);
    (t26069, t26071, t26072, t26073, t26079, t26084, t26123, t27116, t27118, t27120, t27122, t27123)
}
