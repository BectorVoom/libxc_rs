//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1067/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1067<F: Float>(t26519: F, t26653: F, t180: F, t7671: F, t838: F, t2209: F, t2802: F, t233: F, t7684: F, t911: F, t7827: F, t915: F) -> (F, F, F, F, F, F) {
    let t26654 = t26519 + t26653;
    let t26655 = t180 * t26654;
    let t26656 = t838 * t7671;
    let t26658 = t2802 * t2209;
    let t26659 = t233 * t26658;
    let t26660 = t26659 / F::cast_from(16.0_f64);
    let t26662 = t911 * t7684;
    let t26663 = t26662 / F::cast_from(8.0_f64);
    let t26664 = t915 * t7827;
    (t26654, t26655, t26656, t26660, t26663, t26664)
}
