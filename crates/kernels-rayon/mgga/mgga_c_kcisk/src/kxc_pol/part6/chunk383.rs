//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 383/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk383(t1783: f64, t2464: f64, t1310: f64, t1771: f64, t1773: f64, t2449: f64, t2456: f64, t2460: f64, t664: f64, t1791: f64, t1801: f64, t2364: f64) -> (f64, f64, f64, f64, f64) {
    let t2465 = t1783 * t2464;
    let t2466 = t1310 * t2465;
    let t2469 = 0.5397236614853195164e-1_f64 * t2449 * t664 - 0.14392630972941853771e0_f64 * t2456 * t664 + t1771 + 0.17990788716177317213e-1_f64 * t1773 * t2460 - 0.5397236614853195164e-1_f64 * t1773 * t2466;
    let t2470 = t2469 * t1791;
    let t2473 = t1801 * t2364;
    (t2465, t2466, t2469, t2470, t2473)
}
