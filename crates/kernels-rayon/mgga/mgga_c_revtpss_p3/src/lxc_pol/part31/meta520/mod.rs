//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1881;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta520(t1444: f64, t7920: f64, t25924: f64, t1398: f64, t543: f64, t7910: f64, t7301: f64, t1882: f64, t7274: f64, t2022: f64, t5658: f64, t26054: f64, t5722: f64, t1883: f64, t25931: f64, t1955: f64, t7283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27840, t27841, t27845, t27846, t27852, t27853, t27857, t27858, t27861) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1881(t1444, t7920, t25924, t1398, t543, t7910, t7301, t1882, t7274, t2022, t5658, t26054, t5722);
        let (t27864, t27865, t27868) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1882(t1444, t1883, t25931, t1955, t7283);
    (t27840, t27841, t27845, t27846, t27852, t27853, t27857, t27858, t27861, t27864, t27865, t27868)
}
