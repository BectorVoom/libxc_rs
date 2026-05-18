//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1278/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1278<F: Float>(t1615: F, t30424: F, t6176: F, t7429: F, t28714: F, t28741: F, t1394: F, t5644: F, t98409: F, t28356: F, t5649: F, t5655: F) -> (F, F, F, F, F) {
    let t101910 = t6176 * t30424 * t7429 * t1615;
    let t101919 = t28714 * t28741;
    let t101922 = t1394 * t98409 * t5644;
    let t101925 = t1394 * t28356 * t5649;
    let t101928 = t1394 * t28356 * t5655;
    (t101910, t101919, t101922, t101925, t101928)
}
