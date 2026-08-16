//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1191;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta341(t23535: f64, t916: f64, t923: f64, t1600: f64, t6113: f64, t11354: f64, t11358: f64, t11334: f64, t11338: f64, t18919: f64, t18924: f64, t18934: f64, t19002: f64, t19004: f64, t19009: f64, t23521: f64, t23523: f64, t23514: f64, t935: f64, t915: f64, t11387: f64, t23466: f64, t11385: f64, t1642: f64, t19049: f64, t4719: f64, t6223: f64, t1699: f64, t19153: f64, t23448: f64, t23450: f64, t23455: f64, t23459: f64, t23461: f64, t23463: f64, t23465: f64, t23469: f64, t5023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23536, t23538, t23541, t23543, t23545) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1191(t23535, t916, t923, t1600, t6113, t11354, t11358, t11334, t11338, t18919, t18924, t18934, t19002, t19004, t19009, t23521, t23523);
        let (t23546, t23547, t23549, t23550, t23552, t23554, t23556, t23560) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1192(t23514, t23545, t935, t915, t11387, t23466, t11385, t1642, t19049, t4719, t6223, t1699, t19153, t23448, t23450, t23455, t23459, t23461, t23463, t23465, t23469, t5023);
    (t23536, t23538, t23541, t23543, t23546, t23547, t23549, t23550, t23552, t23554, t23556, t23560)
}
