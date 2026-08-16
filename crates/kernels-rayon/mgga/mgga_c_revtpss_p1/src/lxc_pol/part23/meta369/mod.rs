//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1696;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1697;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta369(t1062: f64, t4857: f64, t11986: f64, t1592: f64, t247: f64, t1063: f64, t11940: f64, t3111: f64, t4834: f64, t11788: f64, t3105: f64, t3204: f64, t11262: f64, t1670: f64, t1041: f64, t3172: f64, t4824: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t15707 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1696(t1062, t4857);
        let (t15711, t15712, t15716) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1697(t11986, t1592, t247, t1063, t1062, t11940);
        let (t15724, t15725, t15728, t15731, t15732, t15734, t15736) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1698(t3111, t4834, t1062, t11788, t3105, t3204, t11262, t1670, t1041, t3172, t4824, t3127);
    (t15707, t15711, t15712, t15716, t15724, t15725, t15728, t15731, t15732, t15734, t15736)
}
