//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1221/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1221(t1873: f64, t3938: f64, t671: f64, t3941: f64, t1401: f64, t6534: f64, t577: f64, t7002: f64, t7010: f64, t1184: f64, t460: f64, t33: f64, t3953: f64) -> (f64, f64, f64, f64) {
    let t7014 = 0.135e2_f64 * t3938 * t1873;
    let t7015 = t1873 * t671;
    let t7017 = 27.0_f64 * t3941 * t7015;
    let t7019 = 0.135e2_f64 * t1401 * t6534;
    let t7020 = 0.45e1_f64 * t7002 * t577 + 0.135e2_f64 * t7010 * t671 + t7014 + t7017 + t7019;
    let t7319 = t1184 * t460;
    let t7428 = t3953 * t33;
    (t7015, t7020, t7319, t7428)
}
