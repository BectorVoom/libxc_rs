//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2306/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2306(t1992: f64, t54854: f64, t550: f64, t6976: f64, t26331: f64, t26421: f64, t26446: f64, t3719: f64, t22704: f64, t22705: f64, t26466: f64, t81022: f64, t90806: f64, t90807: f64, t90812: f64, t90816: f64, t90821: f64, t90825: f64, t90829: f64, t90832: f64, t90835: f64, t90837: f64, t90840: f64, t90845: f64, t90848: f64) -> f64 {
    let t90852 = t1992 * t6976 * t54854 * t550;
    let t90856 = t26331 * t26446 * t26421 * t3719;
    let t90859 = t22704 * t22705 * t26466;
    let t90860 = 0.82246703342411321824e-2_f64 * t90859;
    let t90861 = t90806 - 0.12793931631041761173e0_f64 * t90807 - 0.3289868133696452873e-1_f64 * t90812 + 0.3289868133696452873e-1_f64 * t90816 + 0.3289868133696452873e-1_f64 * t90821 - 0.16449340668482264365e-1_f64 * t90825 - 0.3289868133696452873e-1_f64 * t90829 - 0.49348022005446793095e-1_f64 * t90832 + 0.49348022005446793095e-1_f64 * t90835 - 0.52089578783527170489e-1_f64 * t90837 - 0.16449340668482264365e-1_f64 * t90840 - 0.82246703342411321824e-2_f64 * t81022 - t90845 + 0.3289868133696452873e-1_f64 * t90848 - 0.82246703342411321825e-2_f64 * t90852 + 0.49348022005446793095e-1_f64 * t90856 + t90860;
    t90861
}
