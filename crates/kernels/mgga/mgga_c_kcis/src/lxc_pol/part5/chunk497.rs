//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 497/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk497<F: Float>(t1608: F, t2104: F, t286: F, t1597: F, t1599: F, t2096: F, t2100: F, t619: F, t1616: F, t1592: F, t1620: F, t1949: F, t1985: F, t2004: F, t2008: F, t2014: F, t2093: F, t626: F) -> (F, F, F, F, F) {
    let t2105 = t1608 * t2104;
    let t2106 = t286 * t2105;
    let t2109 = -t2096 * t619 / F::new(72.0) + t1597 + t1599 * t2100 / F::new(576.0) - t1599 * t2106 / F::new(192.0);
    let t2110 = t2109 * t1616;
    let t2118 = t2093 * t626 - F::new(0.66725e-1) * t1592 * t2110 + t1620 + F::new(0.11607361111111111111e-2) * t1949 + F::new(0.17411041666666666666e-2) * t1985 - F::new(0.17411041666666666666e-2) * t2004 - F::new(0.46429444444444444443e-2) * t2008 + F::new(0.11607361111111111111e-2) * t2014;
    (t2105, t2106, t2109, t2110, t2118)
}
