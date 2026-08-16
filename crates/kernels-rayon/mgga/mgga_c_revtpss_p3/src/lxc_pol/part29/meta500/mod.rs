//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1816;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta500(t28699: f64, t28729: f64, t28759: f64, t28942: f64, t3: f64, t2055: f64, t670: f64, t1518: f64, t26733: f64, t4292: f64, t7553: f64, t116: f64, t7983: f64, param_d: f64, t117: f64, t28683: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t2113: f64, t2115: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t7547: f64, t7554: f64, t7557: f64, t8118: f64, t8124: f64, t8127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28944, t28945, t28956, t28974, t28975, t28978, t28981, t28986) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1816(t28699, t28729, t28759, t28942, t3, t2055, t670, t1518, t26733, t4292, t7553, t116, t7983, param_d);
        let (t28987, t28990, t28993) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1817(t28986, t670, t117, t28683, t1459, t1461, t1916, t1918, t2113, t2115, t28956, t28975, t28978, t28981, t572, t573, t5795, t5802, t5805, t7547, t7554, t7557, t8118, t8124, t8127);
    (t28944, t28945, t28956, t28974, t28975, t28978, t28981, t28986, t28987, t28990, t28993)
}
