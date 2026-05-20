//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1748;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1749;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta475<F: Float>(t26379: F, t26702: F, t3: F, t2055: F, t2327: F, t116: F, t7373: F, t670: F, t2371: F, t7553: F, t117: F, t26153: F, param_d: F, t1459: F, t1461: F, t2113: F, t2115: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t7547: F, t7554: F, t7557: F, t1518: F, t648: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26703, t26704, t26716, t26730, t26733, t26734, t26737, t26740) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1748::<F>(t26379, t26702, t3, t2055, t2327, t116, t7373, t670, t2371, t7553, t117, t26153, param_d);
        let t26743 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1749::<F>(t1459, t1461, t2113, t2115, t26716, t26730, t26734, t26737, t26740, t4158, t4162, t4165, t572, t573, t7547, t7554, t7557);
        let t27123 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1750::<F>(t1518, t648);
    (t26703, t26704, t26716, t26730, t26733, t26734, t26737, t26740, t26743, t27123)
}
