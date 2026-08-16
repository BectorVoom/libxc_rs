//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 365/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk365(t1589: f64, t2326: f64, t1586: f64, t1578: f64, t1580: f64, t2318: f64, t2322: f64, t535: f64, t541: f64, t1597: f64, t1557: f64, t1601: f64, t2179: f64, t2215: f64, t2234: f64, t2238: f64, t2306: f64, t548: f64) -> (f64, f64, f64, f64, f64) {
    let t2327 = t1589 * t2326;
    let t2328 = t1586 * t2327;
    let t2331 = 0.2698618307426597582e-1_f64 * t2318 * t541 + t1578 + 0.89953943580886586067e-2_f64 * t1580 * t2322 - 0.2698618307426597582e-1_f64 * t535 * t2328;
    let t2332 = t2331 * t1597;
    let t2339 = t2306 * t548 - 0.193e0_f64 * t1557 * t2332 + t1601 + 0.11607361111111111111e-2_f64 * t2179 + 0.17411041666666666666e-2_f64 * t2215 - 0.17411041666666666666e-2_f64 * t2234 + 0.11607361111111111111e-2_f64 * t2238;
    (t2327, t2328, t2331, t2332, t2339)
}
