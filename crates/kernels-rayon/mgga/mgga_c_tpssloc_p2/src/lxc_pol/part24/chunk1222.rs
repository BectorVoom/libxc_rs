//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1222/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1222(t22588: f64, t23861: f64, t3: f64, t112: f64, t7002: f64, t111: f64, t2022: f64, t12521: f64, t1873: f64, t12524: f64, t7015: f64, t3938: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23862 = t22588 + t23861;
    let t23863 = t3 * t23862;
    let t23877 = t7002 * t112;
    let t23880 = t2022 * t111;
    let t23886 = 0.135e2_f64 * t12521 * t1873;
    let t23888 = 54.0_f64 * t12524 * t7015;
    let t23890 = 27.0_f64 * t3938 * t6534;
    (t23862, t23863, t23877, t23880, t23886, t23888, t23890)
}
