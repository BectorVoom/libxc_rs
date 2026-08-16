//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 955/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk955<F: Float>(t11928: F, t1220: F, t15012: F, t15099: F, t15101: F, t15105: F, t15108: F, t15138: F, t1554: F, t1570: F, t1579: F, t17465: F, t17471: F, t17504: F, t17516: F, t17527: F, t17531: F, t17536: F, t17543: F, t3980: F, t4230: F, t4297: F, t4536: F, t5098: F, t5103: F, t5441: F) -> F {
    let t17548 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1220 * t17465 + t17471 - t17504 + t15012 * t1579 / F::cast_from(2.0_f64) + t4536 * t5098 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4536 * t5103 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4230 * t5098 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t4230 * t5103 - F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t4297 * t17516 - t11928 / F::cast_from(9.0_f64) - F::cast_from(0.77534644304710291488e-2_f64) * t3980 * t15138 * t1554 + F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t15099 - F::cast_from(50.0_f64) / F::cast_from(3.0_f64) * t15101 + F::cast_from(20000.0_f64) / F::cast_from(81.0_f64) * t15105 - t17527 - t17531 + F::cast_from(100.0_f64) / F::cast_from(81.0_f64) * t15108 + F::cast_from(4000000.0_f64) / F::cast_from(243.0_f64) * t17536 * t17543 + F::cast_from(44.0_f64) / F::cast_from(3.0_f64) * t1570 * t5441;
    t17548
}
