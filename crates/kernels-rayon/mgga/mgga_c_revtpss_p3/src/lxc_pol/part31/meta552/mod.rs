//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1954;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta552(t114: f64, t7898: f64, t7937: f64, t5542: f64, t7934: f64, t2014: f64, t25826: f64, t5891: f64, t5915: f64, t6998: f64, t25822: f64, t28679: f64, t508: f64, t651: f64, t7935: f64, t2022: f64, t6895: f64, t25924: f64, t1903: f64, t7910: f64, t7296: f64, t6918: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29993, t29996, t29998, t30004) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1954(t114, t7898, t7937, t5542, t7934, t2014, t25826, t5891, t5915, t6998, t25822, t28679);
        let (t30005, t30007, t30015, t30016, t30017, t30020, t30021, t30031) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1955(t30004, t508, t651, t7898, t7935, t2022, t6895, t25924, t1903, t7910, t7296, t6918);
    (t29993, t29996, t29998, t30004, t30005, t30007, t30015, t30016, t30017, t30020, t30021, t30031)
}
