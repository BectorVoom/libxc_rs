//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1124/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1124(t3643: f64, t5272: f64, t11182: f64, t1844: f64, t11229: f64, t1864: f64, t3668: f64, t5358: f64, t286: f64, t69: f64, t3329: f64, t6634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47323 = t5272 * t3643;
    let t47652 = t1844 * t11182;
    let t47700 = t1864 * t11229;
    let t47711 = t5358 * t3668;
    let t61287 = t69 * t286;
    let t63371 = t6634 * t3329;
    (t47323, t47652, t47700, t47711, t61287, t63371)
}
