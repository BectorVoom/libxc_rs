//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2127/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2127(t19451: f64, t6535: f64, t22574: f64, t28830: f64, t31035: f64, t1390: f64, t19631: f64, t1983: f64, t6878: f64, t25989: f64, t91655: f64, t1845: f64, t5356: f64) -> (f64, f64, f64, f64, f64) {
    let t96815 = 2.0_f64 * t19451 * t6535;
    let t96818 = 6.0_f64 * t22574 * t31035 * t28830;
    let t96824 = t1390 * t19631;
    let t96827 = 3.0_f64 * t1983 * t6878 * t96824;
    let t96829 = 6.0_f64 * t91655 * t25989;
    let t96830 = t1845 * t5356;
    (t96815, t96818, t96827, t96829, t96830)
}
