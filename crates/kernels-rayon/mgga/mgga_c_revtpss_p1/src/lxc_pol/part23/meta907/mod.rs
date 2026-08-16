//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta907 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2914;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta907(t11294: f64, t23565: f64, t19128: f64, t4590: f64, t52219: f64, t6145: f64, t23467: f64, t41883: f64, t23547: f64, t2869: f64, t11385: f64, t15396: f64, t6141: f64, t934: f64, t23492: f64, t698: f64, t23471: f64, t141: f64, t77501: f64, t930: f64, t18987: f64, t4606: f64, t15118: f64, t6120: f64, t18950: f64, t4614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77639, t77641, t77643, t77645, t77647, t77657) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2914(t11294, t23565, t19128, t4590, t52219, t6145, t23467, t41883, t23547, t2869, t11385, t15396, t6141, t934);
        let (t77663, t77667, t77670, t77672, t77674, t77676) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2915(t23492, t698, t23471, t141, t77501, t930, t18987, t4606, t15118, t6120, t18950, t4614);
    (t77639, t77641, t77643, t77645, t77647, t77657, t77663, t77667, t77670, t77672, t77674, t77676)
}
