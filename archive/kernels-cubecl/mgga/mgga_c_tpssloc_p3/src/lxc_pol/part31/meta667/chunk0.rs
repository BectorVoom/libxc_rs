//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1960/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1960<F: Float>(t12725: F, t1458: F, t1774: F, t1849: F, t1983: F, t19924: F, t20085: F, t2096: F, t22574: F, t2314: F, t24432: F, t24995: F, t26114: F, t26179: F, t26558: F, t26870: F, t26967: F, t27163: F, t27188: F, t27215: F, t28030: F, t29201: F, t29205: F, t29243: F, t33234: F, t4034: F, t4073: F, t652: F, t6876: F, t7057: F, t7217: F, t74060: F, t7458: F, t7796: F, t7802: F, t9016: F, t97804: F, t97911: F) -> F {
    let t101134 = -F::cast_from(4.0_f64) * t12725 * t7802 - F::cast_from(4.0_f64) * t27188 * t4073 + t97804 * t2096 - F::cast_from(2.0_f64) * t6876 * t29201 - F::cast_from(6.0_f64) * t22574 * t24432 * t74060 + F::cast_from(12.0_f64) * t24995 * t9016 * t19924 + F::cast_from(2.0_f64) * t27215 * t1849 - F::cast_from(2.0_f64) * t28030 * t7057 - F::cast_from(4.0_f64) * t26114 * t7796 - F::cast_from(4.0_f64) * t26179 * t7796 - F::cast_from(4.0_f64) * t7458 * t27163 - F::cast_from(2.0_f64) * t26967 * t1774 + F::cast_from(2.0_f64) * t1983 * t7217 * t20085 - F::cast_from(4.0_f64) * t2314 * t29205 - F::cast_from(4.0_f64) * t4034 * t29205 - F::cast_from(4.0_f64) * t652 * t26870 * t1458 + F::cast_from(2.0_f64) * t6876 * t29243 + F::cast_from(12.0_f64) * t22574 * t26558 * t97911 - F::cast_from(4.0_f64) * t33234 * t4073;
    t101134
}
