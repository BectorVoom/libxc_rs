//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 982/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk982(t2085: f64, t212: f64, t22642: f64, t6890: f64, t214: f64, t7191: f64, t6888: f64, t6891: f64, t22916: f64, t31611: f64, t22751: f64, t31645: f64) -> (f64, f64, f64, f64, f64) {
    let t115330 = t22642 * t212 * t2085 * t6890;
    let t115331 = 0.82246703342411321824e-2_f64 * t115330;
    let t115332 = t214 * t7191;
    let t115334 = t6888 * t115332 * t6891;
    let t115337 = t6888 * t31611 * t22916;
    let t115339 = t22751 * t31645;
    (t115331, t115332, t115334, t115337, t115339)
}
