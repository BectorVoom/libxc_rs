//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2750/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2750(t2659: f64, t57973: f64, t16606: f64, t2379: f64, t39463: f64, t39468: f64, t40714: f64, t40716: f64, t4314: f64, t57959: f64, t57961: f64, t57962: f64, t57966: f64, t57970: f64, t57972: f64) -> (f64, f64) {
    let t57975 = 12.0_f64 * t57973 * t2659;
    let t57976 = 6.0_f64 * t16606 * t2379 * t4314 + t39463 - t39468 - t40714 + t40716 + t57959 + t57961 - t57962 + t57966 + t57970 + t57972 + t57975;
    (t57975, t57976)
}
