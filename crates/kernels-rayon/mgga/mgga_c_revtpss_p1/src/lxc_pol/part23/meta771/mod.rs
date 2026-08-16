//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta771 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2573;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta771(t57421: f64, t1235: f64, t371: f64, t5318: f64, t676: f64, t225: f64, t56331: f64, t1789: f64, t2434: f64, t1012: f64, t44958: f64, t13026: f64, t140: f64, t1222: f64, t1224: f64, t5052: f64, t697: f64, t1260: f64, t44843: f64, t343: f64, t56: f64, t816: f64, t65: f64, t12256: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57422, t57464, t57465, t57471, t57480, t57484) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2573(t57421, t1235, t371, t5318, t676, t225, t56331, t1789, t2434, t1012, t44958, t13026, t140);
        let (t57491, t57520, t57548, t57550) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2574(t1222, t1224, t5052, t697, t1260, t44843, t343, t56, t816, t13026, t65, t12256);
    (t57422, t57464, t57465, t57471, t57480, t57484, t57491, t57520, t57548, t57550)
}
