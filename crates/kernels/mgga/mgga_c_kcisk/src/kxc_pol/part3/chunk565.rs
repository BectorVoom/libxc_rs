//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 565/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk565<F: Float>(t568: F, t967: F, t682: F, t1810: F, t696: F, t1806: F, t1825: F, t143: F, t1849: F, t3290: F, t1060: F, t1814: F, t1824: F, t3293: F, t681: F, t4658: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5082 = t967 * t568;
    let t5084 = 0.46853067927761790996e-2 * t5082 * t682;
    let t5085 = t696 * t1810;
    let t5087 = t1806 * t1825;
    let t5089 = t143 * t1849;
    let t5090 = t682 * t3290;
    let t5093 = t1814 * t1060;
    let t5094 = t5093 * t1824;
    let t5097 = t682 * t3293;
    let t5100 = t681 * t681;
    let t5101 = 1.0 / t5100;
    let t5102 = t5101 * t4658;
    (t5082, t5084, t5085, t5087, t5089, t5090, t5093, t5094, t5097, t5100, t5101, t5102)
}
