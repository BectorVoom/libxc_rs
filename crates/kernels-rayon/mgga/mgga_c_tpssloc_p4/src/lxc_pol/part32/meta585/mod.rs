//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1968;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta585(t24776: f64, t29776: f64, t6256: f64, t7376: f64, t7375: f64, t27516: f64, t8066: f64, t1716: f64, t8077: f64, t1729: f64, t2152: f64, t24589: f64, t24812: f64, t27406: f64, t27507: f64, t27572: f64, t27728: f64, t27737: f64, t29750: f64, t29754: f64, t29759: f64, t29763: f64, t29773: f64, t470: f64, t6168: f64, t7283: f64, t7373: f64, t7999: f64, t8067: f64, t8074: f64, t8078: f64, t8085: f64, t29748: f64, t1241: f64, t2154: f64, t6243: f64, t11606: f64, t24615: f64, t7300: f64, t1409: f64, t1760: f64, t24602: f64, t24601: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29777, t29781, t29782, t29787, t29790, t29793) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1968(t24776, t29776, t6256, t7376, t7375, t27516, t8066, t1716, t8077, t1729, t2152, t24589, t24812, t27406, t27507, t27572, t27728, t27737, t29750, t29754, t29759, t29763, t29773, t470, t6168, t7283, t7373, t7999, t8067, t8074, t8078, t8085);
        let (t29794, t29795, t29798, t29803, t29804, t29808, t29809) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1969(t29748, t29793, t1241, t2154, t6243, t11606, t24615, t7300, t1409, t1760, t24602, t24601);
    (t29777, t29781, t29782, t29787, t29790, t29794, t29795, t29798, t29803, t29804, t29808, t29809)
}
