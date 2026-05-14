//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1039/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1039<F: Float>(t1180: F, t1181: F, t15774: F, t15776: F, t15787: F, t15789: F, t15796: F, t15807: F, t1879: F, t20826: F, t20830: F, t20836: F, t20842: F, t20847: F, t3169: F, t3431: F, t5623: F) -> (F, F) {
    let t20855 = -0.17149607247227894789e-2 * t20826 + 0.85748036236139473945e-2 * t15774 + 0.34299214494455789578e-2 * t15776 - 0.17149607247227894789e-1 * t20830 - 0.17149607247227894789e-2 * t15787 + 0.34299214494455789578e-2 * t20836 - 0.16006300097412701803e-1 * t15789 - 0.17149607247227894789e-2 * t20842 + 0.17149607247227894789e-2 * t20847 - 0.32012600194825403606e-1 * t15796 + 0.80031500487063509016e-2 * t15807 - 0.51448821741683684368e-2 * t1180 * t1181 * t1879 * t3169;
    let t20857 = t3431 * t5623;
    (t20855, t20857)
}
