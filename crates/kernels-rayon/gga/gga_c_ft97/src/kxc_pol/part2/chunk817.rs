//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 817/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk817(t11013: f64, t3499: f64, t12298: f64, t2102: f64, t1775: f64, t3503: f64, t3507: f64, t11755: f64, t11761: f64, t12775: f64, t12778: f64, t12781: f64, t12784: f64, t12788: f64, t12793: f64, t12797: f64, t12800: f64, t12803: f64, t12807: f64, t12809: f64, t12812: f64, t12816: f64, t12817: f64, t12820: f64, t12824: f64, t3051: f64, t3139: f64, t462: f64, t92: f64) -> f64 {
    let t12827 = t3499 * t11013;
    let t12830 = t2102 * t12298;
    let t12834 = 2.0_f64 / 9.0_f64 * t1775 * t3503;
    let t12836 = 4.0_f64 / 9.0_f64 * t1775 * t3507;
    let t12837 = -2.0_f64 / 3.0_f64 * t462 * t12775 - 2.0_f64 / 3.0_f64 * t462 * t12778 - 2.0_f64 * t462 * t12781 + 4.0_f64 / 3.0_f64 * t462 * t12784 - 4.0_f64 / 3.0_f64 * t11761 * t12788 - 4.0_f64 / 3.0_f64 * t11761 * t12793 + 4.0_f64 / 9.0_f64 * t11755 * t12797 + 2.0_f64 / 3.0_f64 * t462 * t12800 + 8.0_f64 / 3.0_f64 * t3139 * t12803 - t92 * t12807 - 4.0_f64 / 9.0_f64 * t12809 - 2.0_f64 / 3.0_f64 * t3051 * t12812 + t12816 + 4.0_f64 / 3.0_f64 * t3139 * t12817 - 2.0_f64 / 9.0_f64 * t462 * t12820 - 10.0_f64 / 27.0_f64 * t462 * t12824 - 8.0_f64 / 9.0_f64 * t3139 * t12827 + t462 * t12830 / 3.0_f64 - t12834 - t12836;
    t12837
}
