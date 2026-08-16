//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 888/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk888(t1799: f64, t28817: f64, t11259: f64, t2364: f64, t8500: f64, t2487: f64, t4609: f64, t8514: f64, t11269: f64, t8504: f64, t2372: f64, t4604: f64) -> (f64, f64, f64, f64, f64) {
    let t28818 = t1799 * t28817;
    let t28822 = t11259 * t2364 * t8500;
    let t28826 = t4609 * t8514 * t2487;
    let t28830 = t11269 * t2364 * t8504;
    let t28834 = t4604 * t8514 * t2372;
    (t28818, t28822, t28826, t28830, t28834)
}
