//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1120/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1120(t1251: f64, t8087: f64, t3598: f64, t225: f64, t497: f64, t5052: f64, t462: f64, t24574: f64, t8006: f64, t3242: f64, t3961: f64, t24601: f64) -> (f64, f64, f64, f64) {
    let t27760 = t8087 * t1251;
    let t27761 = t3598 * t27760;
    let t27766 = t5052 * t225 * t497;
    let t27767 = t462 * t27766;
    let t27770 = t24574 * t8006;
    let t27774 = t497 * t3242;
    let t27775 = t27774 * t3961;
    let t27776 = t24601 * t27775;
    (t27761, t27767, t27770, t27776)
}
