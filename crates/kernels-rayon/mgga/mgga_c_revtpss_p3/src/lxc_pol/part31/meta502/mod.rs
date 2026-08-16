//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta502(t30: f64, t892: f64, t4433: f64, t18875: f64, t25207: f64, t1544: f64, t605: f64, t4343: f64, t1949: f64, t4533: f64, t7071: f64, t689: f64, t7774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27159, t27160, t27166, t27169, t27173, t27182, t27183, t27186) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1822(t30, t892, t4433, t18875, t25207, t1544, t605, t4343, t1949, t4533, t7071, t689, t7774);
    (t27159, t27160, t27166, t27169, t27173, t27182, t27183, t27186)
}
