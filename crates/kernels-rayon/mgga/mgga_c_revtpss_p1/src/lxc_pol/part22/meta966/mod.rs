//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta966 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3229;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta966(t4343: f64, t177: f64, t18550: f64, t762: f64, t50092: f64, t50094: f64, t123: f64, t2630: f64, t5941: f64, t50097: f64, t50099: f64, t14390: f64, t18259: f64, t11075: f64, t14468: f64, t1544: f64, t18268: f64, t18850: f64, t198: f64, t2393: f64, t2394: f64, t2403: f64, t2430: f64, t4541: f64, t4542: f64, t49950: f64, t5966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61234, t61240, t61244, t61245, t61248, t61249, t61250, t61261) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3229(t4343, t177, t18550, t762, t50092, t50094, t123, t2630, t5941, t50097, t50099, t14390, t18259);
        let t61262 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3230(t11075, t14468, t1544, t18268, t18850, t198, t2393, t2394, t2403, t2430, t4541, t4542, t49950, t5966, t61234, t61240, t61244, t61245, t61248, t61249, t61250, t61261);
    (t61234, t61240, t61244, t61245, t61248, t61249, t61250, t61261, t61262)
}
