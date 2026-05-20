//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta978 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3287;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3288;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta978<F: Float>(t50890: F, t18263: F, t2615: F, t50892: F, t50893: F, t40186: F, t40203: F, t40205: F, t14330: F, t18305: F, t2251: F, t50901: F, t40076: F, t40079: F, t40194: F, t40198: F, t62290: F, t62293: F, t62296: F, t62297: F, t62298: F, t62299: F, t62300: F, t14633: F, t14648: F, t14659: F, t14749: F, t1553: F, t1555: F, t18612: F, t225: F, t227: F, t229: F, t2634: F, t2638: F, t2639: F, t2642: F, t4409: F, t4415: F, t4417: F, t4420: F, t6006: F, t6010: F, t6013: F, t61234: F, t61519: F, t62259: F, t62260: F, t62262: F, t62263: F, t62266: F, t62267: F, t62287: F, t73: F, t830: F, t832: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62301, t62303, t62304, t62305, t62306, t62307, t62308, t62311, t62312) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3287::<F>(t50890, t18263, t2615, t50892, t50893, t40186, t40203, t40205, t14330, t18305, t2251, t50901);
        let t62313 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3288::<F>(t40076, t40079, t40194, t40198, t62290, t62293, t62296, t62297, t62298, t62299, t62300, t62301, t62303, t62304, t62305, t62306, t62307, t62308, t62311, t62312);
        let t62347 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3289::<F>(t14633, t14648, t14659, t14749, t1553, t1555, t18612, t225, t227, t229, t2634, t2638, t2639, t2642, t4409, t4415, t4417, t4420, t6006, t6010, t6013, t61234, t61519, t62259, t62260, t62262, t62263, t62266, t62267, t62287, t62313, t73, t830, t832);
    (t62301, t62303, t62304, t62305, t62306, t62307, t62308, t62311, t62312, t62347)
}
