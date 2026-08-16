//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1096/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1096(t39607: f64, t2214: f64, t3293: f64, t528: f64, t132: f64, t1567: f64, t10872: f64, t11686: f64, t10891: f64, t11748: f64, t261: f64, t3304: f64, t7233: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39608 = 0.46574606203128791246e-1_f64 * t39607;
    let t39613 = t3293 * t2214 * t528;
    let t39614 = t132 * t1567;
    let t39627 = t10872 * t11686;
    let t39628 = 0.23115257973478049502e0_f64 * t39627;
    let t39629 = t11748 * t10891;
    let t39630 = 0.69345773920434148506e0_f64 * t39629;
    let t39635 = t3304 * t261 * t7233;
    (t39608, t39613, t39614, t39628, t39630, t39635)
}
