//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1146/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1146<F: Float>(t1615: F, t30424: F, t6176: F, t7429: F, t28714: F, t28741: F, t1394: F, t5644: F, t98409: F, t28356: F, t5649: F, t5655: F, t4153: F, t5663: F, t28499: F, t27636: F, t28844: F, t77834: F, t77844: F, t7968: F, t7978: F, t8222: F, t95009: F, t99013: F) -> (F, F, F, F, F, F) {
    let t101910 = t6176 * t30424 * t7429 * t1615;
    let t101919 = t28714 * t28741;
    let t101922 = t1394 * t98409 * t5644;
    let t101925 = t1394 * t28356 * t5649;
    let t101928 = t1394 * t28356 * t5655;
    let t101931 = t4153 * t28356 * t5663;
    let t101934 = t1394 * t28499 * t5649;
    let t101936 = -0.23168402777777777778e-3 * t99013 * t8222 + 0.46336805555555555556e-3 * t28714 * t28844 + 0.208515625e-2 * t7978 * t6176 * t95009 * t77834 + 0.69505208333333333334e-3 * t7978 * t101910 - 0.13901041666666666667e-2 * t7978 * t6176 * t27636 * t77844 + 0.92754700520833333334e-4 * t7968 * t101910 - 0.7722800925925925926e-4 * t101919 + 0.61905925925925925925e-2 * t101922 - 0.41270617283950617283e-2 * t101925 + 0.12381185185185185185e-1 * t101928 - 0.10317654320987654321e-1 * t101931 + 0.15476481481481481481e-2 * t101934;
    (t101922, t101925, t101928, t101931, t101934, t101936)
}
