//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2311;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta594<F: Float>(t1509: F, t661: F, t26: F, t65: F, t1651: F, t385: F, t1078: F, t1695: F, t1774: F, t494: F, t1276: F, t1828: F) -> (F, F, F, F, F, F) {
        let (t31443, t33127, t33754, t33768, t34934, t34964) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2311::<F>(t1509, t661, t26, t65, t1651, t385, t1078, t1695, t1774, t494, t1276, t1828);
    (t31443, t33127, t33754, t33768, t34934, t34964)
}
