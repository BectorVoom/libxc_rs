//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1179/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1179(t214: f64, t31589: f64, t1985: f64, t22674: f64, t8621: f64, t6897: f64, t2092: f64, t22656: f64, t31106: f64, t31111: f64, t31113: f64, t31115: f64, t31122: f64, t31126: f64, t31585: f64, t3882: f64, t568: f64, t8637: f64) -> (f64, f64, f64, f64) {
    let t31590 = t214 * t31589;
    let t31591 = t1985 * t31590;
    let t31594 = t22674 * t8621;
    let t31595 = t6897 * t31594;
    let t31596 = 0.41123351671205660912e-2_f64 * t31595;
    let t31597 = t31585 * t568 - t31106 + t31111 - t22656 * t2092 - t31113 + 0.82246703342411321825e-2_f64 * t31591 - t3882 * t8637 + t31115 + t31596 - t31122 - t31126;
    (t31590, t31594, t31596, t31597)
}
