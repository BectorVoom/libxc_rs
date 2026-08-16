//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1873/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1873(t13769: f64, t240: f64, t2661: f64, t7269: f64, t13760: f64, t25972: f64, t5609: f64, t7028: f64, t9845: f64, t1889: f64, t94545: f64, t13846: f64, t13877: f64, t7021: f64) -> (f64, f64, f64, f64, f64) {
    let t98152 = t2661 * t7269 * t240 * t13769;
    let t98156 = t25972 * t13760;
    let t98161 = t9845 * t7028 * t5609;
    let t98165 = t94545 * t1889;
    let t98168 = t7021 * t13846 * t13877;
    (t98152, t98156, t98161, t98165, t98168)
}
