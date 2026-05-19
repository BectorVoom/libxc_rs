//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1436/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1436<F: Float>(t1179: F, t11962: F, t12601: F, t15305: F, t15654: F, t15776: F, t15781: F, t15786: F, t17940: F, t18023: F, t18059: F, t18076: F, t18088: F, t18103: F, t35730: F, t35748: F, t4336: F, t438: F, t4444: F, t4457: F, t4464: F, t5096: F, t54777: F, t55164: F, t55194: F, t58322: F, t59568: F, t8196: F, t8511: F, t8516: F, t894: F, t8951: F, t8974: F, t9102: F) -> F {
    let t59963 = -F::cast_from(0.17581974682482873924e4_f64) * t4464 * t55164 * t17940 * t438 + F::cast_from(0.19174369251261266421e6_f64) * t35748 * t18088 - F::cast_from(0.93770531639908660928e4_f64) * t55194 - F::cast_from(0.93568771831764348721e2_f64) * t12601 * t4336 * t15654 * t18023 + F::cast_from(0.779739765264702906e2_f64) * t12601 * t11962 * t15305 * t18023 + F::cast_from(0.34014423178468276542e6_f64) * t9102 * t15781 * t8196 * t15776 + F::cast_from(0.64487301706706172529e0_f64) * t4444 * t18103 + F::cast_from(0.18137053605011111024e0_f64) * t1179 * t894 * t8511 * t58322 + F::cast_from(0.17581974682482873924e4_f64) * t4457 * t15786 * t8974 * t5096 + F::cast_from(0.1343485452223045261e0_f64) * t1179 * t894 * t8951 * t58322 - F::cast_from(0.5373941808892181044e0_f64) * t4444 * t18076 - F::cast_from(0.36282051390366161644e7_f64) * t35730 * t18059 - F::cast_from(0.2686970904446090522e0_f64) * t1179 * t59568 - F::cast_from(0.30228422675018518374e0_f64) * t1179 * t894 * t8516 * t58322 - F::cast_from(0.8790987341241436962e3_f64) * t4464 * t15786 * t54777;
    t59963
}
