//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1066/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1066<F: Float>(t241: F, t41751: F, t2: F, t41536: F, t41448: F, t1775: F, t9939: F, t3139: F, t740: F, t13688: F, t13689: F, t18274: F, t2372: F, t2459: F, t2486: F, t2601: F, t42059: F, t42071: F, t42075: F, t42079: F, t42081: F, t42083: F, t42088: F, t42092: F, t462: F, t737: F, t9692: F, t9947: F, t9952: F) -> (F, F) {
    let t42094 = t41751 * t241;
    let t42095 = t2 * t41536;
    let t42096 = t42095 * t41448;
    let t42100 = t1775 * t9939;
    let t42102 = t3139 * t740;
    let t42104 = -F::cast_from(8.0_f64) * t13688 * t13689 * t42059 - F::cast_from(8.0_f64) * t13688 * t18274 * t2601 * t2459 + F::cast_from(8.0_f64) * t462 * t2372 * t9947 * t9692 + F::cast_from(8.0_f64) * t462 * t737 * t42071 + F::cast_from(2.0_f64) * t462 * t737 * t42075 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t42079 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t42081 - F::cast_from(8.0_f64) * t462 * t2486 * t42083 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t462 * t9952 * t42088 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t42092 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t462 * t42094 * t42096 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t42100 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t42102;
    (t42096, t42104)
}
