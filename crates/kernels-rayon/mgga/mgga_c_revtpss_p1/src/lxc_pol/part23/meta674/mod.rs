//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2410;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta674(t11627: f64, t42859: f64, t342: f64, t12077: f64, t989: f64, t12153: f64, t3057: f64, t1071: f64, t11200: f64, t3494: f64, t3519: f64, t13026: f64, t240: f64, t3361: f64, t1146: f64, t9303: f64, t2304: f64, t25273: f64, t268: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43536, t43537, t43574, t43598, t43637, t43752, t43764) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2410(t11627, t42859, t342, t12077, t989, t12153, t3057, t1071, t11200, t3494, t3519, t13026, t240);
        let (t43766, t43771, t43776, t43813) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2411(t3361, t1146, t9303, t2304, t25273, t268, t404);
    (t43536, t43537, t43574, t43598, t43637, t43752, t43764, t43766, t43771, t43776, t43813)
}
