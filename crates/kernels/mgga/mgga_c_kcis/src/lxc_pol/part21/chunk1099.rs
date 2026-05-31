//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1099/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1099<F: Float>(t26946: F, t7754: F, t26918: F, t26920: F, t26922: F, t26925: F, t26927: F, t26931: F, t26934: F, t26936: F, t26939: F, t26942: F, t26944: F) -> (F, F) {
    let t26947 = t7754 * t26946;
    let t26949 = -t26918 / F::cast_from(16.0_f64) + t26920 / F::cast_from(16.0_f64) + F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t26922 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t26925 - t26927 / F::cast_from(12.0_f64) + t26931 / F::cast_from(48.0_f64) - t26934 / F::cast_from(8.0_f64) - t26936 / F::cast_from(3.0_f64) + t26939 / F::cast_from(12.0_f64) + t26942 / F::cast_from(24.0_f64) - t26944 / F::cast_from(128.0_f64) - t26947 / F::cast_from(72.0_f64);
    (t26947, t26949)
}
