//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 441/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk441<F: Float>(t1651: F, t407: F, t1603: F, t182: F, t1170: F, t119: F, t1226: F, t1228: F, t1229: F, t1231: F, t1235: F, t1238: F, t1242: F, t1246: F, t151: F, t1530: F, t1627: F, t1631: F, t1633: F, t1636: F, t1639: F, t1642: F, t1645: F, t1649: F) -> (F, F, F) {
    let t1652 = t1651 * t407;
    let t1655 = t182 * t1603;
    let t1658 = t1226 - t1228 - F::new(0.65854491829355115987e0) * t1229 + F::new(0.65854491829355115987e0) * t1231 + t1235 + F::new(0.65854491829355115987e0) * t1238 - F::new(0.65854491829355115987e0) * t1242 - t1246 - F::new(0.65854491829355115987e0) * t1627 + F::new(0.65854491829355115987e0) * t1631 + F::new(0.13170898365871023197e1) * t1530 * t1633 - F::new(0.65854491829355115987e0) * t151 * t1636 - F::new(0.65854491829355115987e0) * t151 * t1639 - F::new(0.65854491829355115987e0) * t1170 * t1642 + F::new(0.65854491829355115987e0) * t1645 - F::new(0.65854491829355115987e0) * t1649 - F::new(0.65854491829355115987e0) * t151 * t1652 + F::new(0.65854491829355115987e0) * t119 * t1655;
    (t1652, t1655, t1658)
}
