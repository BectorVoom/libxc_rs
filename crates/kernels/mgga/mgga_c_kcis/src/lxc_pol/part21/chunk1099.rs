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
    let t26949 = -t26918 / F::new(16.0) + t26920 / F::new(16.0) + F::new(11.0) / F::new(18.0) * t26922 - F::new(2.0) / F::new(9.0) * t26925 - t26927 / F::new(12.0) + t26931 / F::new(48.0) - t26934 / F::new(8.0) - t26936 / F::new(3.0) + t26939 / F::new(12.0) + t26942 / F::new(24.0) - t26944 / F::new(128.0) - t26947 / F::new(72.0);
    (t26947, t26949)
}
