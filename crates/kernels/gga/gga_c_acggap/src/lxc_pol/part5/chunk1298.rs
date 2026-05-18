//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1298/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1298<F: Float>(t1165: F, t1173: F, t1181: F, t1531: F, t1532: F, t1552: F, t1759: F, t18690: F, t18702: F, t18704: F, t1899: F, t20138: F, t24110: F, t24113: F, t24128: F, t24130: F, t24138: F, t3396: F, t4267: F, t4450: F, t5116: F, t839: F, t945: F) -> F {
    let t24141 = -F::new(0.34299214494455789578e-2) * t1173 * t1165 * t1552 * t1759 * t839 + F::new(0.68598428988911579156e-2) * t24110 - F::new(0.51448821741683684367e-2) * t4450 * t1165 * t1532 * t24113 + F::new(0.51448821741683684367e-2) * t1531 * t1165 * t1532 * t20138 + F::new(0.68026775414003982663e-1) * t18690 + F::new(0.17149607247227894789e-2) * t18702 - F::new(0.85748036236139473944e-3) * t1531 * t1165 * t1899 * t945 - F::new(0.34299214494455789578e-1) * t24128 + F::new(0.12004725073059526353e-1) * t24130 + F::new(0.13719685797782315831e-1) * t3396 * t1181 * t4267 * t5116 - F::new(0.85748036236139473944e-2) * t24138 + F::new(0.17149607247227894789e-2) * t18704;
    t24141
}
