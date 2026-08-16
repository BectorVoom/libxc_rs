//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1034/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1034(t3258: f64, t6514: f64, t7832: f64, t8430: f64, t1227: f64, t937: f64, t2363: f64, t3199: f64, t410: f64, t6523: f64, t8436: f64, t2393: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8507 = t6514 * t3258;
    let t8508 = t7832 * t8430;
    let t8511 = t937 * t1227;
    let t8512 = t2363 * t8511;
    let t8515 = t410 * t3199;
    let t8516 = t2363 * t8515;
    let t8519 = t6523 * t3258;
    let t8520 = t7832 * t8436;
    let t8529 = t2393 * t8511;
    (t8507, t8508, t8511, t8512, t8515, t8516, t8519, t8520, t8529)
}
