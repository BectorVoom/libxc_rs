//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta978 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3287;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3288;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta978(t50890: f64, t18263: f64, t2615: f64, t50892: f64, t50893: f64, t40186: f64, t40203: f64, t40205: f64, t14330: f64, t18305: f64, t2251: f64, t50901: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t62290: f64, t62293: f64, t62296: f64, t62297: f64, t62298: f64, t62299: f64, t62300: f64, t14633: f64, t14648: f64, t14659: f64, t14749: f64, t1553: f64, t1555: f64, t18612: f64, t225: f64, t227: f64, t229: f64, t2634: f64, t2638: f64, t2639: f64, t2642: f64, t4409: f64, t4415: f64, t4417: f64, t4420: f64, t6006: f64, t6010: f64, t6013: f64, t61234: f64, t61519: f64, t62259: f64, t62260: f64, t62262: f64, t62263: f64, t62266: f64, t62267: f64, t62287: f64, t73: f64, t830: f64, t832: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62301, t62303, t62304, t62305, t62306, t62307, t62308, t62311, t62312) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3287(t50890, t18263, t2615, t50892, t50893, t40186, t40203, t40205, t14330, t18305, t2251, t50901);
        let t62313 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3288(t40076, t40079, t40194, t40198, t62290, t62293, t62296, t62297, t62298, t62299, t62300, t62301, t62303, t62304, t62305, t62306, t62307, t62308, t62311, t62312);
        let t62347 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3289(t14633, t14648, t14659, t14749, t1553, t1555, t18612, t225, t227, t229, t2634, t2638, t2639, t2642, t4409, t4415, t4417, t4420, t6006, t6010, t6013, t61234, t61519, t62259, t62260, t62262, t62263, t62266, t62267, t62287, t62313, t73, t830, t832);
    (t62301, t62303, t62304, t62305, t62306, t62307, t62308, t62311, t62312, t62347)
}
