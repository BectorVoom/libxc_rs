//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2079/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2079(t1354: f64, t91285: f64, t26298: f64, t80958: f64, t22779: f64, t26319: f64, t1358: f64, t26248: f64, t3862: f64, t7715: f64, t22705: f64, t22852: f64, t236: f64, t5286: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91286 = t91285 * t1354;
    let t91287 = 7.0_f64 / 1152.0_f64 * t91286;
    let t91290 = t80958 * t26298;
    let t91300 = t22779 * t26319;
    let t91301 = 0.56521858531796547196e-2_f64 * t91300;
    let t91303 = t26248 * t1358;
    let t91304 = 7.0_f64 / 1152.0_f64 * t91303;
    let t91305 = t7715 * t3862;
    let t91310 = t22852 * t22705 * t236 * t5286 * t550;
    (t91287, t91290, t91301, t91304, t91305, t91310)
}
