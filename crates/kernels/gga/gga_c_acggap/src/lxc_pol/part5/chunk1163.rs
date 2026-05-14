//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1163/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1163<F: Float>(t3379: F, t5618: F, t1410: F, t2937: F, t406: F, t16899: F, t6324: F, t3409: F, t6086: F, t1101: F, t1165: F, t1889: F, t4282: F, t1173: F, t1181: F, t1531: F, t1532: F, t1552: F, t1759: F, t18690: F, t18702: F, t18704: F, t1899: F, t20138: F, t3396: F, t4267: F, t4450: F, t5116: F, t839: F, t945: F) -> (F, F) {
    let t24110 = t3379 * t5618;
    let t24112 = t2937 * t1410;
    let t24113 = t24112 * t406;
    let t24128 = t16899 * t6324;
    let t24130 = t3409 * t6086;
    let t24138 = t4282 * t1165 * t1889 * t1101;
    let t24141 = -0.34299214494455789578e-2 * t1173 * t1165 * t1552 * t1759 * t839 + 0.68598428988911579156e-2 * t24110 - 0.51448821741683684367e-2 * t4450 * t1165 * t1532 * t24113 + 0.51448821741683684367e-2 * t1531 * t1165 * t1532 * t20138 + 0.68026775414003982663e-1 * t18690 + 0.17149607247227894789e-2 * t18702 - 0.85748036236139473944e-3 * t1531 * t1165 * t1899 * t945 - 0.34299214494455789578e-1 * t24128 + 0.12004725073059526353e-1 * t24130 + 0.13719685797782315831e-1 * t3396 * t1181 * t4267 * t5116 - 0.85748036236139473944e-2 * t24138 + 0.17149607247227894789e-2 * t18704;
    (t24113, t24141)
}
