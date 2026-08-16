//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2110/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2110(t27674: f64, t3548: f64, t15753: f64, t7310: f64, t27608: f64, t7321: f64, t1222: f64, t27586: f64, t3540: f64, t8049: f64, t2132: f64, t2136: f64, t3966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95511 = t27674 * t3548 / 162.0_f64;
    let t95512 = t7310 * t15753;
    let t95515 = 0.20186378047070195428e-3_f64 * t27608 * t7321;
    let t95517 = t27586 * t1222 / 1152.0_f64;
    let t95520 = t8049 * t3540;
    let t95540 = 0.20186378047070195428e-3_f64 * t2132 * t3966 * t2136;
    (t95511, t95512, t95515, t95517, t95520, t95540)
}
