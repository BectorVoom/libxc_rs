//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 897/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk897(t18: f64, t713: f64, t2600: f64, t2599: f64, t766: f64, t2607: f64, t2606: f64, t1882: f64, t3999: f64, t3995: f64, t1175: f64, t2373: f64, t2574: f64) -> (f64, f64, f64, f64, f64) {
    let t13892 = t18 * t713;
    let t13893 = t2600 * t13892;
    let t13894 = t2599 * t13893;
    let t13897 = t18 * t766;
    let t13898 = t2607 * t13897;
    let t13899 = t2606 * t13898;
    let t13903 = 2.0_f64 / 9.0_f64 * t1882 * t3999;
    let t13905 = 2.0_f64 / 9.0_f64 * t1882 * t3995;
    let t13907 = t2574 * t1175 * t2373;
    (t13894, t13899, t13903, t13905, t13907)
}
