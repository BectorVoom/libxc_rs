//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 815/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk815(t2612: f64, t5676: f64, t2611: f64, t549: f64, t2033: f64, t1391: f64, t2723: f64, t825: f64, t2013: f64, t2607: f64, t6574: f64, t823: f64) -> (f64, f64, f64, f64, f64) {
    let t7792 = t5676 * t2612;
    let t7794 = t549 * t2611;
    let t7795 = t2033 * t7794;
    let t7797 = t1391 * t2723;
    let t7798 = t825 * t7797;
    let t7800 = t2013 * t2607;
    let t7802 = t823 * t6574;
    (t7792, t7795, t7798, t7800, t7802)
}
