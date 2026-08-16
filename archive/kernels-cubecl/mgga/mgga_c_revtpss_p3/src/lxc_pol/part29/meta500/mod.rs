//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1816;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta500<F: Float>(t28699: F, t28729: F, t28759: F, t28942: F, t3: F, t2055: F, t670: F, t1518: F, t26733: F, t4292: F, t7553: F, t116: F, t7983: F, param_d: F, t117: F, t28683: F, t1459: F, t1461: F, t1916: F, t1918: F, t2113: F, t2115: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t7547: F, t7554: F, t7557: F, t8118: F, t8124: F, t8127: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28944, t28945, t28956, t28974, t28975, t28978, t28981, t28986) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1816::<F>(t28699, t28729, t28759, t28942, t3, t2055, t670, t1518, t26733, t4292, t7553, t116, t7983, param_d);
        let (t28987, t28990, t28993) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1817::<F>(t28986, t670, t117, t28683, t1459, t1461, t1916, t1918, t2113, t2115, t28956, t28975, t28978, t28981, t572, t573, t5795, t5802, t5805, t7547, t7554, t7557, t8118, t8124, t8127);
    (t28944, t28945, t28956, t28974, t28975, t28978, t28981, t28986, t28987, t28990, t28993)
}
