//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta586(t23514: f64, t23545: f64, t935: f64, t915: f64, t11387: f64, t23466: f64, t11385: f64, t1642: f64, t19049: f64, t4719: f64, t6223: f64, t1699: f64, t19153: f64, t23448: f64, t23450: f64, t23455: f64, t23459: f64, t23461: f64, t23463: f64, t23465: f64, t23469: f64, t5023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23546, t23547, t23549, t23550, t23552, t23554, t23556, t23560) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2215(t23514, t23545, t935, t915, t11387, t23466, t11385, t1642, t19049, t4719, t6223, t1699, t19153, t23448, t23450, t23455, t23459, t23461, t23463, t23465, t23469, t5023);
    (t23546, t23547, t23549, t23550, t23552, t23554, t23556, t23560)
}
