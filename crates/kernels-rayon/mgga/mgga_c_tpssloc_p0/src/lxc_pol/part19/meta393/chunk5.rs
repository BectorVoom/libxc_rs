//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1496/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1496(t12513: f64, t12537: f64, t1396: f64, t1398: f64, t1404: f64, t3: f64, t39022: f64, t39024: f64, t39026: f64, t39028: f64, t3932: f64, t3946: f64, t45546: f64, t45580: f64, t580: f64) -> f64 {
    let tv4rho40 = t3 * t45546 * t580 + 4.0_f64 * t12513 * t1404 + 4.0_f64 * t12537 * t1396 + t1398 * t45580 + 6.0_f64 * t3932 * t3946 + 4.0_f64 * t39022 + 12.0_f64 * t39024 + 12.0_f64 * t39026 + 4.0_f64 * t39028;
    tv4rho40
}
