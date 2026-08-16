//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 920/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk920(t115539: f64, t2085: f64, t213: f64, t225: f64, t22642: f64, t22643: f64, t8621: f64, t22716: f64, t8612: f64, t22724: f64, t31569: f64, t1862: f64, t8308: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115540 = 0.26044789391763585244e-1_f64 * t115539;
    let t115545 = t213 * t2085 * t225;
    let t115550 = t22642 * t22643 * t8621;
    let t115551 = 0.82246703342411321824e-2_f64 * t115550;
    let t115566 = t22716 * t8612;
    let t115567 = 0.63969658155208805863e-1_f64 * t115566;
    let t115629 = t22724 * t31569;
    let t115630 = 0.26044789391763585244e-1_f64 * t115629;
    let t115833 = t8308 * t1862;
    (t115540, t115545, t115551, t115567, t115630, t115833)
}
