//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2602/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2602(t10471: f64, t52834: f64, t11737: f64, t11651: f64, t15507: f64, t13969: f64, t15621: f64, t3506: f64, t11791: f64, t5005: f64, t11697: f64, t15477: f64, t3577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52835 = t52834 * t10471;
    let t52836 = t52835 * t11737;
    let t52845 = t15507 * t11651;
    let t52859 = t3506 * t13969 * t15621;
    let t52872 = t5005 * t11791;
    let t52875 = t3577 * t11697 * t15477;
    (t52835, t52836, t52845, t52859, t52872, t52875)
}
