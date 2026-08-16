//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1196/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1196(t533: f64, t6995: f64, t1390: f64, t1983: f64, t1388: f64, t3701: f64, t2019: f64, t1873: f64, t3938: f64, t671: f64, t3941: f64, t1401: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6996 = t533 * t6995;
    let t6997 = t6996 * t1390;
    let t6998 = t1983 * t6997;
    let t6999 = t3701 * t1388;
    let t7000 = t2019 * t6999;
    let t7001 = t1983 * t7000;
    let t7014 = 0.135e2_f64 * t3938 * t1873;
    let t7015 = t1873 * t671;
    let t7017 = 27.0_f64 * t3941 * t7015;
    let t7019 = 0.135e2_f64 * t1401 * t6534;
    (t6996, t6997, t6998, t6999, t7000, t7001, t7014, t7015, t7017, t7019)
}
