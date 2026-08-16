//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 953/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk953(t31206: f64, t6897: f64, t794: f64, t1985: f64, t1998: f64, t214: f64, t22870: f64, t22716: f64, t8480: f64, t31203: f64, t6914: f64, t2006: f64, t3791: f64) -> (f64, f64, f64, f64, f64) {
    let t114097 = t6897 * t794 * t31206;
    let t114098 = 0.16449340668482264365e-1_f64 * t114097;
    let t114102 = 0.16449340668482264365e-1_f64 * t1985 * t214 * t1998 * t22870;
    let t114104 = 0.12793931631041761173e0_f64 * t22716 * t8480;
    let t114105 = t6914 * t31203;
    let t114106 = 0.76763589786250567036e-1_f64 * t114105;
    let t114107 = t2006 * t3791;
    (t114098, t114102, t114104, t114106, t114107)
}
