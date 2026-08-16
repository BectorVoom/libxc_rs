//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1155/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1155(t1181: f64, t3456: f64, t3457: f64, t360: f64, t5852: f64, t1165: f64, t372: f64, t5922: f64, t1180: f64, t15774: f64, t15776: f64, t15787: f64, t15789: f64, t15796: f64, t15807: f64, t1879: f64, t20826: f64, t20830: f64, t20836: f64, t3169: f64) -> f64 {
    let t20842 = t3456 * t1181 * t5852 * t3457 * t360;
    let t20847 = t3456 * t1165 * t5922 * t3457 * t372;
    let t20855 = -0.17149607247227894789e-2_f64 * t20826 + 0.85748036236139473945e-2_f64 * t15774 + 0.34299214494455789578e-2_f64 * t15776 - 0.17149607247227894789e-1_f64 * t20830 - 0.17149607247227894789e-2_f64 * t15787 + 0.34299214494455789578e-2_f64 * t20836 - 0.16006300097412701803e-1_f64 * t15789 - 0.17149607247227894789e-2_f64 * t20842 + 0.17149607247227894789e-2_f64 * t20847 - 0.32012600194825403606e-1_f64 * t15796 + 0.80031500487063509016e-2_f64 * t15807 - 0.51448821741683684368e-2_f64 * t1180 * t1181 * t1879 * t3169;
    t20855
}
