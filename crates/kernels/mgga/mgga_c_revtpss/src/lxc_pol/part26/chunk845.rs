//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 845/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk845<F: Float>(t1032: F, t3043: F, t1040: F, t1065: F, t3075: F, t906: F, t1042: F, t1047: F, t1063: F, t1068: F, t11977: F, t11980: F, t11983: F, t11989: F, t11991: F, t11994: F, t11999: F, t12004: F, t12007: F, t12010: F, t12013: F, t12017: F, t3115: F, t3127: F, t3130: F, t3157: F, t3164: F) -> (F,) {
    let t12020 = t3043 * t1032;
    let t12021 = t12020 * t1040;
    let t12024 = t1065 * t3075;
    let t12025 = t12024 * t906;
    let t12026 = t1042 * t12025;
    let t12029 = -0.68598428988911579154e-2 * t11977 * t1047 + 0.85748036236139473944e-3 * t11980 + 0.71456696863449561621e-3 * t1063 * t11983 - 0.95275595817932748825e-4 * t11989 + 0.42874018118069736972e-3 * t11991 * t1068 - 0.85748036236139473944e-3 * t11994 * t3130 + 0.34299214494455789577e-2 * t11999 * t3164 + 0.14481890564325777821e-1 * t12004 * t1068 - 0.30488190661738479624e-2 * t12007 + 0.85748036236139473944e-3 * t12010 - 0.68598428988911579154e-2 * t12013 * t3157 - 0.64311027177104605458e-3 * t3115 * t12017 + 0.64311027177104605458e-3 * t12021 * t1047 - 0.42874018118069736972e-3 * t3127 * t12026;
    (t12029,)
}
