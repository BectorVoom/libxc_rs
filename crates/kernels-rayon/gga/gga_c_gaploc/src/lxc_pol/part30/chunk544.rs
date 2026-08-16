//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 544/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk544(t1457: f64, t2788: f64, t1445: f64, t2779: f64, t2787: f64, t447: f64, t528: f64, t999: f64, t1: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2862 = t1457 * t2788;
    let t2865 = t1445 * t2779;
    let t2868 = t2787 * t447;
    let t2869 = t1445 * t2868;
    let t2872 = t528 * t999;
    let t2875 = t986 * t1;
    (t2862, t2865, t2868, t2869, t2872, t2875)
}
