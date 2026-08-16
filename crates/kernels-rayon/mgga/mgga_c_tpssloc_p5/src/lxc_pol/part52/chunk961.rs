//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 961/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk961(t1937: f64, t23442: f64, t1926: f64, t3158: f64, t40: f64, t6722: f64, t6712: f64, t995: f64, t1942: f64, t3082: f64, t344: f64, t1009: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23443 = t23442 * t1937;
    let t23447 = t1926 * t3158 / 432.0_f64;
    let t23448 = t6722 * t40;
    let t23449 = t23448 * t1937;
    let t23463 = t6712 * t995;
    let t23469 = t1942 * t3082 / 6912.0_f64;
    let t23470 = t40 * t344;
    let t23471 = t23470 * t1009;
    (t23443, t23447, t23449, t23463, t23469, t23470, t23471)
}
