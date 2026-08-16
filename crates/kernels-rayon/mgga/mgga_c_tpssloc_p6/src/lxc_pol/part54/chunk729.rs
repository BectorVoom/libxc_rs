//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 729/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk729(t2019: f64, t6999: f64, t1983: f64, t1873: f64, t3938: f64, t671: f64, t3941: f64, t1401: f64, t6534: f64, t33: f64, t63: f64, t2240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7000 = t2019 * t6999;
    let t7001 = t1983 * t7000;
    let t7014 = 0.135e2_f64 * t3938 * t1873;
    let t7015 = t1873 * t671;
    let t7017 = 27.0_f64 * t3941 * t7015;
    let t7019 = 0.135e2_f64 * t1401 * t6534;
    let t7025 = t33 * t63;
    let t7026 = t2240 * t7025;
    (t7000, t7001, t7014, t7015, t7017, t7019, t7025, t7026)
}
