//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 909/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk909(t2218: f64, t2345: f64, t2206: f64, t651: f64, t2348: f64, t2215: f64, t123: f64, t727: f64, t2349: f64, t2192: f64, t737: f64, t767: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8218 = t2218 * t2345;
    let t8220 = t651 * t2206;
    let t8222 = 0.16265371950452609763e-1_f64 * t2348 * t8220;
    let t8223 = t651 * t2215;
    let t8225 = 0.48159733137676571078e0_f64 * t2348 * t8223;
    let t8226 = t727 * t123;
    let t8227 = t8226 * t2349;
    let t8229 = t2192 * t737;
    let t8231 = 0.21687162600603479684e-1_f64 * t2348 * t8229;
    let t8232 = t651 * t2345;
    let t8234 = 0.32530743900905219526e-1_f64 * t2348 * t8232;
    let t8274 = t767 * t767;
    (t8218, t8220, t8222, t8223, t8225, t8227, t8229, t8231, t8232, t8234, t8274)
}
