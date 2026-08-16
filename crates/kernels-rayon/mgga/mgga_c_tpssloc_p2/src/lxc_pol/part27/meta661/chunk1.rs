//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2315/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2315(t90983: f64, t1336: f64, t1352: f64, t16033: f64, t16055: f64, t1825: f64, t22879: f64, t26404: f64, t26442: f64, t26453: f64, t26458: f64, t3773: f64, t3777: f64, t3851: f64, t5234: f64, t5344: f64, t7747: f64, t81199: f64, t90942: f64, t90946: f64, t90952: f64, t90957: f64, t90962: f64, t90964: f64, t90968: f64, t90971: f64, t90980: f64) -> f64 {
    let t90984 = 0.82246703342411321824e-2_f64 * t90983;
    let t90985 = -2.0_f64 * t5344 * t90942 * t1352 - 2.0_f64 * t5344 * t90946 * t1352 + 4.0_f64 * t16055 * t26453 - 2.0_f64 * t1336 * t90952 * t1352 + t90957 - t1336 * t26458 * t3851 - t90962 - t90964 + t3773 * t7747 + 0.16449340668482264365e-1_f64 * t90968 + t90971 - t5234 * t22879 - t1336 * t81199 * t1825 - 2.0_f64 * t3777 * t26442 - 2.0_f64 * t16033 * t26404 + 0.82246703342411321824e-2_f64 * t90980 + t90984;
    t90985
}
