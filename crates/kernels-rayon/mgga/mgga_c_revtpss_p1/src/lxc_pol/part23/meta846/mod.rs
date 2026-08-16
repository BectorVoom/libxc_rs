//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta846 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2726;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2727;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta846(t17303: f64, t5323: f64, t12866: f64, t5406: f64, t58895: f64, t17789: f64, t21306: f64, t17401: f64, t17617: f64, t15687: f64, t17394: f64, t3782: f64, t17708: f64, t59948: f64, t370: f64, t17727: f64, t12916: f64, t21258: f64, t3718: f64, t17753: f64, t21045: f64, t5401: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70583, t70612, t70616, t70623, t70629, t70630) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2726(t17303, t5323, t12866, t5406, t58895, t17789, t21306, t17401, t17617, t15687, t17394, t3782);
        let (t70639, t70646, t70647, t70664, t70667, t70672) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2727(t17708, t59948, t17394, t370, t17727, t12916, t21258, t3718, t17753, t21045, t12866, t5401, t58895);
    (t70583, t70612, t70616, t70623, t70629, t70630, t70639, t70646, t70647, t70664, t70667, t70672)
}
