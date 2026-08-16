//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 801/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk801(t6553: f64, t8335: f64, t1880: f64, t1894: f64, t59: f64) -> (f64, f64, f64) {
    let t8336 = t6553 * t8335;
    let t8338 = 0.16449340668482264365e-1_f64 * t1880 * t8336;
    let t8339 = t1894 * t59;
    (t8336, t8338, t8339)
}
