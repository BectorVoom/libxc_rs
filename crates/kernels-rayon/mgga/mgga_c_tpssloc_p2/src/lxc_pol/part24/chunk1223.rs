//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1223/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1223(t16535: f64, t1873: f64, t6534: f64, t671: f64, t3941: f64, t2363: f64, t1401: f64, t22479: f64, t2319: f64, t23862: f64, t23877: f64, t23880: f64, t23886: f64, t23888: f64, t23890: f64, t577: f64, t7010: f64) -> (f64, f64, f64) {
    let t23892 = 27.0_f64 * t16535 * t1873;
    let t23893 = t6534 * t671;
    let t23895 = 54.0_f64 * t3941 * t23893;
    let t23896 = t1873 * t2363;
    let t23898 = 27.0_f64 * t3941 * t23896;
    let t23900 = 0.135e2_f64 * t1401 * t22479;
    let t23901 = 0.45e1_f64 * t23862 * t577 + 27.0_f64 * t23877 * t671 + 27.0_f64 * t23880 * t2319 + 0.135e2_f64 * t7010 * t2363 + t23886 + t23888 + t23890 + t23892 + t23895 + t23898 + t23900;
    (t23893, t23896, t23901)
}
