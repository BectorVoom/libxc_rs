//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1748;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1749;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta475(t26379: f64, t26702: f64, t3: f64, t2055: f64, t2327: f64, t116: f64, t7373: f64, t670: f64, t2371: f64, t7553: f64, t117: f64, t26153: f64, param_d: f64, t1459: f64, t1461: f64, t2113: f64, t2115: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, t7547: f64, t7554: f64, t7557: f64, t1518: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26703, t26704, t26716, t26730, t26733, t26734, t26737, t26740) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1748(t26379, t26702, t3, t2055, t2327, t116, t7373, t670, t2371, t7553, t117, t26153, param_d);
        let t26743 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1749(t1459, t1461, t2113, t2115, t26716, t26730, t26734, t26737, t26740, t4158, t4162, t4165, t572, t573, t7547, t7554, t7557);
        let t27123 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1750(t1518, t648);
    (t26703, t26704, t26716, t26730, t26733, t26734, t26737, t26740, t26743, t27123)
}
