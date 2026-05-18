//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 365/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk365<F: Float>(t1589: F, t2326: F, t1586: F, t1578: F, t1580: F, t2318: F, t2322: F, t535: F, t541: F, t1597: F, t1557: F, t1601: F, t2179: F, t2215: F, t2234: F, t2238: F, t2306: F, t548: F) -> (F, F, F, F, F) {
    let t2327 = t1589 * t2326;
    let t2328 = t1586 * t2327;
    let t2331 = F::new(0.2698618307426597582e-1) * t2318 * t541 + t1578 + F::new(0.89953943580886586067e-2) * t1580 * t2322 - F::new(0.2698618307426597582e-1) * t535 * t2328;
    let t2332 = t2331 * t1597;
    let t2339 = t2306 * t548 - F::new(0.193e0) * t1557 * t2332 + t1601 + F::new(0.11607361111111111111e-2) * t2179 + F::new(0.17411041666666666666e-2) * t2215 - F::new(0.17411041666666666666e-2) * t2234 + F::new(0.11607361111111111111e-2) * t2238;
    (t2327, t2328, t2331, t2332, t2339)
}
