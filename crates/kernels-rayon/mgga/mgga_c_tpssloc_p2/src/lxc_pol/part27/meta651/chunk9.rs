//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2273/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2273(t5: f64, t90107: f64, t90135: f64, t90167: f64, t90199: f64, t90230: f64, t90265: f64, t90315: f64, t90346: f64, t112: f64, t2319: f64, t7450: f64, t26117: f64, t6534: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t90350 = piecewise3(t8, 0.0_f64, t90107 + t90135 + t90167 + t90199 + t90230 + t90265 + t90315 + t90346);
    let t90351 = t90350 * t112;
    let t90352 = t7450 * t2319;
    let t90355 = 4.0_f64 * t26117 * t6534;
    (t90351, t90352, t90355)
}
