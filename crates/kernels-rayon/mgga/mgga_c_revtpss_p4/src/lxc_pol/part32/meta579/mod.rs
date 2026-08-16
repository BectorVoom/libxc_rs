//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1906;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta579(t1445: f64, t28824: f64, t689: f64, t102274: f64, t25878: f64, t102100: f64, t26069: f64, t26231: f64, t98380: f64, t13730: f64, t2098: f64, t2782: f64, t102315: f64, t25899: f64, t2439: f64, t8099: f64, t94391: f64, t102234: f64, t3916: f64, t25895: f64, t2097: f64, t9990: f64, t102115: f64, t7289: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102361, t102363, t102364, t102367, t102372) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1906(t1445, t28824, t689, t102274, t25878, t102100, t26069, t26231, t98380, t13730, t2098, t2782);
        let (t102378, t102385, t102386, t102394, t102396, t102397, t102404) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1907(t102315, t25899, t2439, t8099, t94391, t102234, t3916, t25895, t2097, t9990, t102115, t7289);
    (t102361, t102363, t102364, t102367, t102372, t102378, t102385, t102386, t102394, t102396, t102397, t102404)
}
