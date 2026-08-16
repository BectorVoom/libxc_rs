//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1966;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta557(t29991: f64, t30159: f64, t3: f64, t2042: f64, t6941: f64, t1916: f64, t7950: f64, t7953: f64, t1936: f64, t5883: f64, t572: f64, t1518: f64, t28276: f64, param_d: f64, t5920: f64, t7330: f64, t117: f64, t30004: f64, t1918: f64, t2040: f64, t573: f64, t6945: f64, t6948: f64, t7944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30160, t30161, t30171, t30180, t30182, t30184, t30185, t30187, t30188) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1966(t29991, t30159, t3, t2042, t6941, t1916, t7950, t7953, t1936, t5883, t572, t1518, t28276, param_d);
        let (t30191, t30194, t30197) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1967(t30188, t572, t5920, t7330, t117, t30004, t1918, t2040, t30171, t30180, t30182, t30184, t30187, t573, t6945, t6948, t7944);
    (t30160, t30161, t30171, t30185, t30188, t30191, t30194, t30197)
}
