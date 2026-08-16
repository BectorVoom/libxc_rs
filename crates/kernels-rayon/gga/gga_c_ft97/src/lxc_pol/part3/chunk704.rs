//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 704/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk704(t1557: f64, t586: f64, t1037: f64, t1771: f64, t3524: f64, t458: f64, t2: f64, t9224: f64, t1775: f64, t3503: f64, t3507: f64, t3500: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12796 = t586 * t1557;
    let t12809 = t1771 * t1037;
    let t12816 = 2.0_f64 / 3.0_f64 * t458 * t3524;
    let t12823 = t9224 * t2;
    let t12834 = 2.0_f64 / 9.0_f64 * t1775 * t3503;
    let t12836 = 4.0_f64 / 9.0_f64 * t1775 * t3507;
    let t12839 = 4.0_f64 / 27.0_f64 * t1775 * t3500;
    (t12796, t12809, t12816, t12823, t12834, t12836, t12839)
}
