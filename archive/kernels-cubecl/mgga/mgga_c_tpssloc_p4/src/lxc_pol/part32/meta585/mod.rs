//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1968;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta585<F: Float>(t24776: F, t29776: F, t6256: F, t7376: F, t7375: F, t27516: F, t8066: F, t1716: F, t8077: F, t1729: F, t2152: F, t24589: F, t24812: F, t27406: F, t27507: F, t27572: F, t27728: F, t27737: F, t29750: F, t29754: F, t29759: F, t29763: F, t29773: F, t470: F, t6168: F, t7283: F, t7373: F, t7999: F, t8067: F, t8074: F, t8078: F, t8085: F, t29748: F, t1241: F, t2154: F, t6243: F, t11606: F, t24615: F, t7300: F, t1409: F, t1760: F, t24602: F, t24601: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29777, t29781, t29782, t29787, t29790, t29793) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1968::<F>(t24776, t29776, t6256, t7376, t7375, t27516, t8066, t1716, t8077, t1729, t2152, t24589, t24812, t27406, t27507, t27572, t27728, t27737, t29750, t29754, t29759, t29763, t29773, t470, t6168, t7283, t7373, t7999, t8067, t8074, t8078, t8085);
        let (t29794, t29795, t29798, t29803, t29804, t29808, t29809) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1969::<F>(t29748, t29793, t1241, t2154, t6243, t11606, t24615, t7300, t1409, t1760, t24602, t24601);
    (t29777, t29781, t29782, t29787, t29790, t29794, t29795, t29798, t29803, t29804, t29808, t29809)
}
