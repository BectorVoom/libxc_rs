//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1066/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1066(t241: f64, t41751: f64, t2: f64, t41536: f64, t41448: f64, t1775: f64, t9939: f64, t3139: f64, t740: f64, t13688: f64, t13689: f64, t18274: f64, t2372: f64, t2459: f64, t2486: f64, t2601: f64, t42059: f64, t42071: f64, t42075: f64, t42079: f64, t42081: f64, t42083: f64, t42088: f64, t42092: f64, t462: f64, t737: f64, t9692: f64, t9947: f64, t9952: f64) -> (f64, f64) {
    let t42094 = t41751 * t241;
    let t42095 = t2 * t41536;
    let t42096 = t42095 * t41448;
    let t42100 = t1775 * t9939;
    let t42102 = t3139 * t740;
    let t42104 = -8.0_f64 * t13688 * t13689 * t42059 - 8.0_f64 * t13688 * t18274 * t2601 * t2459 + 8.0_f64 * t462 * t2372 * t9947 * t9692 + 8.0_f64 * t462 * t737 * t42071 + 2.0_f64 * t462 * t737 * t42075 - 8.0_f64 / 9.0_f64 * t42079 - 16.0_f64 / 27.0_f64 * t42081 - 8.0_f64 * t462 * t2486 * t42083 + 40.0_f64 / 9.0_f64 * t462 * t9952 * t42088 - 16.0_f64 / 9.0_f64 * t42092 - 80.0_f64 / 81.0_f64 * t462 * t42094 * t42096 + 4.0_f64 / 9.0_f64 * t42100 + 112.0_f64 / 81.0_f64 * t42102;
    (t42096, t42104)
}
