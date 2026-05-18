//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 497/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk497<F: Float>(t568: F, t967: F, t682: F, t143: F, t1849: F, t681: F, t1394: F, t429: F, t686: F, t3841: F, t435: F, t690: F) -> (F, F, F, F, F, F, F) {
    let t5082 = t967 * t568;
    let t5084 = F::new(0.46853067927761790996e-2) * t5082 * t682;
    let t5089 = t143 * t1849;
    let t5100 = t681 * t681;
    let t5101 = F::new(1.0) / t5100;
    let t5122 = F::new(0.8197e-2) * t429 * t1394 * t686;
    let t5125 = F::new(0.21133333333333333333e-2) * t435 * t3841 * t690;
    (t5082, t5084, t5089, t5100, t5101, t5122, t5125)
}
