//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1170/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1170(t10238: f64, t2649: f64, t2745: f64, t2892: f64, t317: f64, t44131: f64, t44262: f64, t44272: f64, t44352: f64, t44362: f64, t44483: f64, t44603: f64, t44718: f64, t44736: f64, t44751: f64, t44767: f64, t44781: f64, t788: f64, t829: f64, t880: f64) -> f64 {
    let t44789 = -6.0_f64 * t2745 * t2892 - 8.0_f64 * t10238 * t880 - 12.0_f64 * t44272 - 8.0_f64 * t44483 - 6.0_f64 * t2649 * t2892 - t44718 * t829 * t317 + 48.0_f64 * t44603 - 72.0_f64 * t44262 - t788 * (t44736 + t44751 + t44767 + t44781) * t317 - 2.0_f64 * t44131 - 48.0_f64 * t44352 + 48.0_f64 * t44362;
    t44789
}
