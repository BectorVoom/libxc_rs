//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1019/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1019(t670: f64, t7724: f64, t32342: f64, t575: f64, t1464: f64, t8602: f64, t136: f64, t32141: f64, t10309: f64, t2247: f64, t6957: f64, t84: f64, t8440: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t108120 = t7724 * t670;
    let t119422 = t32342 * t575;
    let t119424 = t8602 * t1464;
    let t119443 = t32141 * t136;
    let t119444 = t10309 * t119443;
    let t119451 = t2247 * t6957 * t136;
    let t119456 = t2247 * t119443;
    let t119457 = t8440 * t84;
    (t108120, t119422, t119424, t119444, t119451, t119456, t119457)
}
