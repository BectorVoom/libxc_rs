//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 751/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk751(t5: f64, t27991: f64, t112: f64, t1868: f64, t5456: f64, t1873: f64, t19451: f64, t1441: f64, t1458: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t27992 = piecewise3(t8, 0.0_f64, t27991);
    let t27993 = t27992 * t112;
    let t27996 = t1868 * t5456;
    let t28001 = 2.0_f64 * t19451 * t1873;
    let t28002 = t1441 * t1458;
    (t27992, t27993, t27996, t28001, t28002)
}
