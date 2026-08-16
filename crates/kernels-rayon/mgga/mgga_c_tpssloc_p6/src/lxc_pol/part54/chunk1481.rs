//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1481/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1481(t2098: f64, t8119: f64, t1851: f64, t8852: f64, t117407: f64, t117410: f64, t117412: f64, t117416: f64, t117420: f64, t117422: f64, t124673: f64, t124687: f64, t125024: f64, t125029: f64, t125043: f64, t125046: f64, t1398: f64, t1852: f64, t1858: f64, t2099: f64, t2170: f64, t27286: f64, t27930: f64, t3: f64, t32393: f64, t32415: f64, t580: f64, t7416: f64, t7961: f64) -> f64 {
    let t125050 = t2098 * t8119;
    let t125053 = t1851 * t8852;
    let t125058 = t117422 + t2099 * t27930 + t124673 + t1398 * (t124687 + t125029 + t125043 + t125046) + t125050 + t117410 + t117416 + t32393 * t1858 + t117412 + t1852 * t32415 + t125053 + t117420 + t117407 + t2170 * t27286 + t3 * t125024 * t580 + t7416 * t7961;
    t125058
}
