//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 957/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk957(t1454: f64, t626: f64, t1453: f64, t2331: f64, t666: f64, t1444: f64, t2341: f64, t659: f64, t2: f64, t95: f64, t584: f64, t1449: f64, t2349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4041 = t626 * t1454;
    let t4043 = t2331 * t1453;
    let t4044 = t4043 * t666;
    let t4049 = t2341 * t1444;
    let t4050 = t4049 * t659;
    let t4053 = t95 * t2;
    let t4054 = t4053 * t584;
    let t4059 = t2349 * t1449;
    (t4041, t4043, t4044, t4049, t4050, t4053, t4054, t4059)
}
