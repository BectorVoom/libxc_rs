//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1435/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1435<F: Float>(t1162: F, t1170: F, t123: F, t1520: F, t1536: F, t15889: F, t15911: F, t17714: F, t18161: F, t27630: F, t438: F, t4444: F, t4450: F, t4457: F, t449: F, t450: F, t5337: F, t5364: F, t54111: F, t55162: F, t55164: F, t55176: F, t55181: F, t55214: F, t55337: F, t59023: F, t59434: F, t59458: F, t59527: F, t894: F, t914: F, t935: F) -> F {
    let t59916 = F::cast_from(0.28977204965962526182e-1_f64) * t1162 * t914 * t59458 + F::cast_from(0.13909058383662012568e1_f64) * t1162 * t914 * t59527 - F::cast_from(0.20420978873790287968e1_f64) * t15889 * t5337 - F::cast_from(0.49917948358154037253e1_f64) * t55214 * t1520 - F::cast_from(0.64487301706706172529e0_f64) * t4444 * t17714 + F::cast_from(0.23184437530160156653e8_f64) * t27630 * t450 * t59023 * t935 + F::cast_from(0.5848048239485271795e1_f64) * t1170 * t894 * t449 * t59434 * t438 + F::cast_from(0.1343485452223045261e-1_f64) * t55162 - F::cast_from(0.21495767235568724176e0_f64) * t55176 - F::cast_from(0.30909018630360027928e0_f64) * t55181 - F::cast_from(0.52888765211949381121e1_f64) * t55337 * t1536 - F::cast_from(0.18545411178216016757e1_f64) * t4450 * t18161 - F::cast_from(0.3399992049339603072e1_f64) * t15911 * t5364 + F::cast_from(0.35163949364965747848e4_f64) * t4457 * t55164 * t54111 * t123;
    t59916
}
