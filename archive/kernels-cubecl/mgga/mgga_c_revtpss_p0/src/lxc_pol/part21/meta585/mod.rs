//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2301;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2302;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta585<F: Float>(t13547: F, t18176: F, t3: F, t1518: F, t2327: F, t116: F, t4292: F, t670: F, t2371: F, t5801: F, t117: F, t13514: F, param_d: F, t1459: F, t1461: F, t1916: F, t1918: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18177, t18178, t18190, t18204, t18207, t18208, t18211, t18214) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2301::<F>(t13547, t18176, t3, t1518, t2327, t116, t4292, t670, t2371, t5801, t117, t13514, param_d);
        let t18217 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2302::<F>(t1459, t1461, t18190, t18204, t18208, t18211, t18214, t1916, t1918, t4158, t4162, t4165, t572, t573, t5795, t5802, t5805);
    (t18177, t18178, t18190, t18204, t18207, t18208, t18211, t18214, t18217)
}
