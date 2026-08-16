//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1155/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1155<F: Float>(t1181: F, t3456: F, t3457: F, t360: F, t5852: F, t1165: F, t372: F, t5922: F, t1180: F, t15774: F, t15776: F, t15787: F, t15789: F, t15796: F, t15807: F, t1879: F, t20826: F, t20830: F, t20836: F, t3169: F) -> F {
    let t20842 = t3456 * t1181 * t5852 * t3457 * t360;
    let t20847 = t3456 * t1165 * t5922 * t3457 * t372;
    let t20855 = -F::cast_from(0.17149607247227894789e-2_f64) * t20826 + F::cast_from(0.85748036236139473945e-2_f64) * t15774 + F::cast_from(0.34299214494455789578e-2_f64) * t15776 - F::cast_from(0.17149607247227894789e-1_f64) * t20830 - F::cast_from(0.17149607247227894789e-2_f64) * t15787 + F::cast_from(0.34299214494455789578e-2_f64) * t20836 - F::cast_from(0.16006300097412701803e-1_f64) * t15789 - F::cast_from(0.17149607247227894789e-2_f64) * t20842 + F::cast_from(0.17149607247227894789e-2_f64) * t20847 - F::cast_from(0.32012600194825403606e-1_f64) * t15796 + F::cast_from(0.80031500487063509016e-2_f64) * t15807 - F::cast_from(0.51448821741683684368e-2_f64) * t1180 * t1181 * t1879 * t3169;
    t20855
}
