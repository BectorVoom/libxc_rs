//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 928/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk928(t125: f64, t3664: f64, t3671: f64, t8313: f64, t1385: f64, t8130: f64, t2383: f64, t3689: f64, t2143: f64, t3622: f64, t1369: f64, t8176: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10590 = t125 * t3664;
    let t10600 = 7.0_f64 / 2304.0_f64 * t8313 * t3671;
    let t10617 = t8130 * t1385;
    let t10620 = 7.0_f64 / 576.0_f64 * t2383 * t3689;
    let t10630 = 7.0_f64 / 72.0_f64 * t2143 * t3622;
    let t10635 = t8176 * t1369;
    (t10590, t10600, t10617, t10620, t10630, t10635)
}
