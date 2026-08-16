//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2420;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta680(t1121: f64, t13045: f64, t606: f64, t221: f64, t461: f64, t462: f64, t624: f64, t1250: f64, t1235: f64, t1236: f64, t2434: f64, t371: f64, t12625: f64, t458: f64, t456: f64, t225: f64, t43813: f64, t126: f64, t13099: f64, t1224: f64, t12268: f64, t1222: f64, t1226: f64, t2438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44738, t44797, t44799, t44829) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2420(t1121, t13045, t606, t221, t461, t462, t624, t1250, t1235, t1236, t2434, t371);
        let (t44842, t44843, t44865, t44895, t44919, t44931) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2421(t12625, t458, t456, t225, t43813, t126, t13099, t1224, t12268, t1222, t1226, t2438);
    (t44738, t44797, t44799, t44829, t44842, t44843, t44865, t44895, t44919, t44931)
}
