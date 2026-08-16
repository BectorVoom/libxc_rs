//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta585(t23535: f64, t916: f64, t923: f64, t1600: f64, t6113: f64, t11354: f64, t11358: f64, t11334: f64, t11338: f64, t18919: f64, t18924: f64, t18934: f64, t19002: f64, t19004: f64, t19009: f64, t23521: f64, t23523: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t23536, t23538, t23540, t23541, t23543, t23545) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2214(t23535, t916, t923, t1600, t6113, t11354, t11358, t11334, t11338, t18919, t18924, t18934, t19002, t19004, t19009, t23521, t23523);
    (t23536, t23538, t23540, t23541, t23543, t23545)
}
