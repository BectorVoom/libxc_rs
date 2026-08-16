//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta416(t14586: f64, t14786: f64, t14791: f64, t1559: f64, t4433: f64, t14785: f64, t2652: f64, t6030: f64, t10858: f64, t6024: f64, t10816: f64, t10824: f64, t10826: f64, t18456: f64, t18459: f64, t18462: f64, t18466: f64, t18471: f64, t18475: f64, t2745: f64, t4362: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18477, t18478, t18481, t18482, t18485, t18487, t18489) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1798(t14586, t14786, t14791, t1559, t4433, t14785, t2652, t6030, t10858, t6024, t10816, t10824, t10826, t18456, t18459, t18462, t18466, t18471, t18475, t2745, t4362);
    (t18477, t18478, t18481, t18482, t18485, t18487, t18489)
}
