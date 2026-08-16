//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1814;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1815;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1816;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta514<F: Float>(t30570: F, t508: F, t1518: F, t8065: F, t29494: F, t7488: F, t2107: F, t22483: F, t26161: F, t29498: F, t2051: F, t5883: F, t1312: F, t18245: F, t2055: F, t28653: F, t30138: F, t30143: F, t30553: F, t4248: F, t5920: F, t7359: F, t7889: F, t7983: F, t28938: F, t7900: F, t22475: F, t1502: F, t1519: F, t1843: F, t2014: F, t2052: F, t2089: F, t30558: F, t30563: F, t569: F, t5877: F, t5884: F, t5921: F, t651: F, t6765: F, t7732: F, t7969: F, t7984: F, t7988: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t30571, t30578, t30581, t30584, t30586, t30589) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1814::<F>(t30570, t508, t1518, t8065, t29494, t7488, t2107, t22483, t26161, t29498, t2051, t5883);
        let t30612 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1815::<F>(t1312, t1518, t18245, t2055, t28653, t30138, t30143, t30553, t30570, t30589, t4248, t5920, t7359, t7889, t7983);
        let (t30614, t30617, t30625) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1816::<F>(t28938, t7900, t2107, t22475, t1502, t1519, t1843, t2014, t2052, t2089, t28653, t30558, t30563, t30571, t30578, t30581, t30584, t30586, t30589, t30612, t4248, t508, t569, t5877, t5884, t5921, t651, t6765, t7359, t7732, t7969, t7984, t7988, t8065);
    (t30571, t30578, t30581, t30584, t30586, t30589, t30612, t30614, t30617, t30625)
}
