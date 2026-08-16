//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 634/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk634(t3784: f64, t3789: f64, t2660: f64, t3717: f64, t2767: f64, t3636: f64, t3641: f64, t3647: f64, t3653: f64, t209: f64, t1112: f64, t3480: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3790 = t3784 * t3789;
    let t3792 = t2660 * t3717;
    let t3793 = t3792 * t2767;
    let t3855 = 0.2429468532550759923e-3_f64 * t3636 - 0.17379648562707520765e-3_f64 * t3641 - 0.50613927761474165061e-5_f64 * t3647 + 0.10862280351692200478e-4_f64 * t3653;
    let t3856 = t3855 * t209;
    let t3858 = 2.0_f64 * t3480 * t1112;
    (t3790, t3792, t3793, t3855, t3856, t3858)
}
