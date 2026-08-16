//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2148;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta581(t10883: f64, t10884: f64, t248: f64, t3101: f64, t10473: f64, t361: f64, t363: f64, t42342: f64, t42345: f64, t3131: f64, t3047: f64, t3077: f64, t10908: f64, t3114: f64, t1036: f64, t10438: f64, t221: f64, t339: f64, t42813: f64, t10283: f64, t995: f64, t10931: f64, t135: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43285, t43288, t43291, t43292, t43298) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2148(t10883, t10884, t248, t3101, t10473, t361, t363, t42342, t42345, t3131, t3047, t3077);
        let (t43301, t43303, t43307, t43310, t43313) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2149(t10908, t3114, t1036, t10438, t221, t339, t42813, t10283, t995, t10931, t135, t973);
    (t43285, t43288, t43291, t43292, t43298, t43301, t43303, t43307, t43310, t43313)
}
