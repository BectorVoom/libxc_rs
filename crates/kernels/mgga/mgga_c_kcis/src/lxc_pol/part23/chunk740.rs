//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 740/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk740<F: Float>(t206: F, t8797: F, t8912: F, t209: F, t880: F, t208: F, t214: F, t2733: F, t2742: F, t2748: F, t876: F, t8769: F, t8782: F, t8785: F, t8788: F, t884: F) -> (F, F, F) {
    let t210 = F::cast_from(0.0_f64) < t206;
    let t8913 = t8797 + t8912;
    let t8915 = piecewise3::<F>(t210, t8913, -t8913);
    let t8917 = t209 * t880 * t8915;
    let t8920 = -F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t8769 * t214 - F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t2733 * t884 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t876 * t2742 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t876 * t2748 - t208 * t8782 / F::cast_from(16.0_f64) + t8785 * t8788 / F::cast_from(16.0_f64) - t208 * t8917 / F::cast_from(96.0_f64);
    (t8913, t8915, t8920)
}
