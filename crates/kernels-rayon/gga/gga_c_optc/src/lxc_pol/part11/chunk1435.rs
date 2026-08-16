//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1435/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1435(t1162: f64, t1170: f64, t123: f64, t1520: f64, t1536: f64, t15889: f64, t15911: f64, t17714: f64, t18161: f64, t27630: f64, t438: f64, t4444: f64, t4450: f64, t4457: f64, t449: f64, t450: f64, t5337: f64, t5364: f64, t54111: f64, t55162: f64, t55164: f64, t55176: f64, t55181: f64, t55214: f64, t55337: f64, t59023: f64, t59434: f64, t59458: f64, t59527: f64, t894: f64, t914: f64, t935: f64) -> f64 {
    let t59916 = 0.28977204965962526182e-1_f64 * t1162 * t914 * t59458 + 0.13909058383662012568e1_f64 * t1162 * t914 * t59527 - 0.20420978873790287968e1_f64 * t15889 * t5337 - 0.49917948358154037253e1_f64 * t55214 * t1520 - 0.64487301706706172529e0_f64 * t4444 * t17714 + 0.23184437530160156653e8_f64 * t27630 * t450 * t59023 * t935 + 0.5848048239485271795e1_f64 * t1170 * t894 * t449 * t59434 * t438 + 0.1343485452223045261e-1_f64 * t55162 - 0.21495767235568724176e0_f64 * t55176 - 0.30909018630360027928e0_f64 * t55181 - 0.52888765211949381121e1_f64 * t55337 * t1536 - 0.18545411178216016757e1_f64 * t4450 * t18161 - 0.3399992049339603072e1_f64 * t15911 * t5364 + 0.35163949364965747848e4_f64 * t4457 * t55164 * t54111 * t123;
    t59916
}
