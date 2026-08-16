//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 541/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk541(t228: f64, t2532: f64, t2535: f64, t2541: f64, t2627: f64, t2764: f64, t2766: f64, t2771: f64, t2772: f64, t2789: f64, t899: f64, t906: f64) -> f64 {
    let t2791 = t228 * t2764 - 2.0_f64 * t2766 * t906 + 2.0_f64 * t2771 * t2772 - t2789 * t899 - t2532 + t2535 - t2541 + t2627;
    t2791
}
