//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta966 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3229;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta966<F: Float>(t4343: F, t177: F, t18550: F, t762: F, t50092: F, t50094: F, t123: F, t2630: F, t5941: F, t50097: F, t50099: F, t14390: F, t18259: F, t11075: F, t14468: F, t1544: F, t18268: F, t18850: F, t198: F, t2393: F, t2394: F, t2403: F, t2430: F, t4541: F, t4542: F, t49950: F, t5966: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61234, t61240, t61244, t61245, t61248, t61249, t61250, t61261) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3229::<F>(t4343, t177, t18550, t762, t50092, t50094, t123, t2630, t5941, t50097, t50099, t14390, t18259);
        let t61262 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3230::<F>(t11075, t14468, t1544, t18268, t18850, t198, t2393, t2394, t2403, t2430, t4541, t4542, t49950, t5966, t61234, t61240, t61244, t61245, t61248, t61249, t61250, t61261);
    (t61234, t61240, t61244, t61245, t61248, t61249, t61250, t61261, t61262)
}
