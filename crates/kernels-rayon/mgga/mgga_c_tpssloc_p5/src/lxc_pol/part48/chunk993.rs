//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 993/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk993(t1985: f64, t22662: f64, t31611: f64, t31560: f64, t6914: f64, t113950: f64, t113956: f64, t113961: f64, t113963: f64, t114140: f64, t115368: f64, t115372: f64, t115378: f64, t115417: f64, t115436: f64, t115480: f64, t115498: f64, t12033: f64, t1375: f64, t1378: f64, t2091: f64, t22904: f64, t24082: f64, t24139: f64, t26224: f64, t31653: f64, t3887: f64, t3912: f64, t6958: f64, t6962: f64, t6993: f64, t8627: f64, t93818: f64) -> f64 {
    let t115506 = t1985 * t31611 * t22662;
    let t115508 = t6914 * t31560;
    let t115513 = -0.82246703342411321825e-2_f64 * t115368 - t31653 * t3912 + 0.49348022005446793095e-1_f64 * t115372 - 12.0_f64 * t26224 * t93818 * t6962 + 0.16449340668482264365e-1_f64 * t115378 + t113950 + 2.0_f64 * t1375 * t3887 * t2091 * t22904 - t1375 * t1378 * (t115417 + t115436 + t115480 + t115498) - 2.0_f64 * t24082 * t6993 - t113956 - 0.82246703342411321825e-2_f64 * t115506 - t113961 - t113963 - 0.76763589786250567036e-1_f64 * t115508 + 2.0_f64 * t12033 * t8627 + t114140 - t6958 * t24139;
    t115513
}
