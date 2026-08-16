//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1998/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1998<F: Float>(t101138: F, t101150: F, t101951: F, t102102: F, t113: F, t1442: F, t15868: F, t19451: F, t1983: F, t22574: F, t24175: F, t26161: F, t26163: F, t26558: F, t26559: F, t26870: F, t26902: F, t26906: F, t26974: F, t28821: F, t28834: F, t28969: F, t29197: F, t29377: F, t29378: F, t5107: F, t650: F, t6876: F, t6879: F, t6999: F, t7050: F, t7218: F, t7685: F, t7787: F, t7940: F, t91655: F, t92169: F, t96797: F, t97875: F, t97894: F) -> F {
    let t102105 = -t1983 * t29377 * t6999 + F::cast_from(4.0_f64) * t26161 * t101138 * t26163 + F::cast_from(6.0_f64) * t22574 * t26558 * t97894 - F::cast_from(6.0_f64) * t91655 * t26974 - F::cast_from(2.0_f64) * t1442 * t26870 - t650 * t29197 + F::cast_from(3.0_f64) * t1983 * t101150 * t6879 - F::cast_from(2.0_f64) * t19451 * t7050 + t6876 * t29378 - F::cast_from(6.0_f64) * t26161 * t92169 * t97875 + t28821 * t7218 - F::cast_from(2.0_f64) * t1983 * t7940 * t15868 - F::cast_from(2.0_f64) * t7685 * t26902 - F::cast_from(2.0_f64) * t7787 * t5107 + F::cast_from(4.0_f64) * t96797 * t26559 + F::cast_from(6.0_f64) * t7685 * t26906 + F::cast_from(3.0_f64) * t1983 * t24175 * t28834 + F::cast_from(3.0_f64) * t6876 * t28969 - t113 * (t101951 + t102102);
    t102105
}
