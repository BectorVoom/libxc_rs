//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta530(t4292: f64, t93: f64, t1936: f64, t7002: f64, t7889: f64, t2322: f64, t7741: f64, t5523: f64, t1312: f64, t28042: f64, t2042: f64, t5795: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t28219, t28221, t28223, t28225, t28227, t28229, t28257) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1945(t4292, t93, t1936, t7002, t7889, t2322, t7741, t5523, t1312, t28042, t2042, t5795);
    (t28219, t28221, t28223, t28225, t28227, t28229, t28257)
}
