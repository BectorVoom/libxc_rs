//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1083/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1083<F: Float>(t10143: F, t1882: F, t10007: F, t10039: F, t10044: F, t10075: F, t13885: F, t14200: F, t1901: F, t2373: F, t2409: F, t2413: F, t242: F, t2568: F, t2569: F, t2574: F, t2579: F, t2619: F, t41414: F, t41435: F, t42546: F, t446: F, t713: F, t724: F, t761: F, t773: F, t9787: F) -> F {
    let t42557 = t1882 * t10143;
    let t42563 = -F::cast_from(8.0_f64) * t1901 * t13885 * t761 * t713 * t10044 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t9787 * t10075 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t10007 * t2409 * t2579 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t14200 * t41435 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t724 * t2619 * t2413 - F::cast_from(12.0_f64) * t446 * t242 * t41414 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t42546 + F::cast_from(8.0_f64) * t446 * t2574 * t2568 * t2373 * t2569 + F::cast_from(8.0_f64) * t446 * t2574 * t773 * t10039 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t42557 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t724 * t2619 * t2409;
    t42563
}
