//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1292/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1292<F: Float>(t1709: F, t4566: F, t829: F, t95557: F, t18482: F, t4947: F, t922: F, t100407: F, t100420: F, t100423: F, t100426: F, t101003: F, t101012: F, t101018: F, t101053: F, t26685: F, t7703: F, t93592: F, t95868: F) -> (F, F, F) {
    let t101195 = t95557 * t4566 * t1709 * t829;
    let t101208 = t4947 * t18482 * t922;
    let t101213 = F::cast_from(0.22109259259259259259e-2_f64) * t95868 + F::cast_from(0.11054629629629629629e-2_f64) * t100407 + F::cast_from(0.61782407407407407408e-3_f64) * t93592 * t101195 - F::cast_from(0.27802083333333333334e-2_f64) * t7703 * t101053 + F::cast_from(0.22109259259259259259e-2_f64) * t100420 - F::cast_from(0.66327777777777777776e-2_f64) * t100423 - F::cast_from(0.22109259259259259259e-2_f64) * t100426 + F::cast_from(0.46336805555555555556e-3_f64) * t7703 * t101012 + F::cast_from(0.92673611111111111112e-3_f64) * t7703 * t101018 - F::cast_from(0.92754700520833333333e-4_f64) * t26685 * t101208 - F::cast_from(0.92754700520833333333e-4_f64) * t26685 * t101003;
    (t101195, t101208, t101213)
}
