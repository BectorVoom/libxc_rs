//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1814;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1815;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1816;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta514(t30570: f64, t508: f64, t1518: f64, t8065: f64, t29494: f64, t7488: f64, t2107: f64, t22483: f64, t26161: f64, t29498: f64, t2051: f64, t5883: f64, t1312: f64, t18245: f64, t2055: f64, t28653: f64, t30138: f64, t30143: f64, t30553: f64, t4248: f64, t5920: f64, t7359: f64, t7889: f64, t7983: f64, t28938: f64, t7900: f64, t22475: f64, t1502: f64, t1519: f64, t1843: f64, t2014: f64, t2052: f64, t2089: f64, t30558: f64, t30563: f64, t569: f64, t5877: f64, t5884: f64, t5921: f64, t651: f64, t6765: f64, t7732: f64, t7969: f64, t7984: f64, t7988: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30571, t30578, t30581, t30584, t30586, t30589) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1814(t30570, t508, t1518, t8065, t29494, t7488, t2107, t22483, t26161, t29498, t2051, t5883);
        let t30612 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1815(t1312, t1518, t18245, t2055, t28653, t30138, t30143, t30553, t30570, t30589, t4248, t5920, t7359, t7889, t7983);
        let (t30614, t30617, t30625) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1816(t28938, t7900, t2107, t22475, t1502, t1519, t1843, t2014, t2052, t2089, t28653, t30558, t30563, t30571, t30578, t30581, t30584, t30586, t30589, t30612, t4248, t508, t569, t5877, t5884, t5921, t651, t6765, t7359, t7732, t7969, t7984, t7988, t8065);
    (t30571, t30578, t30581, t30584, t30586, t30589, t30612, t30614, t30617, t30625)
}
