//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 842/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk842<F: Float>(t395: F, t8140: F, t1508: F, t971: F, t1251: F, t156: F, t2885: F, t496: F, t1243: F, t2890: F, t128: F, t8102: F, t10: F, t5749: F, t5751: F, t5753: F, t5755: F, t5759: F, t5764: F, t5768: F, t5776: F, t8117: F, t8118: F, t8126: F, t8127: F, t8131: F, t8137: F, t8139: F) -> (F, F, F, F, F) {
    let t8142 = 0.97434166666666666666e0 * t8140 * t395;
    let t8143 = t1508 * t971;
    let t8144 = t8143 * t1251;
    let t8145 = 0.32478055555555555555e0 * t8144;
    let t8146 = t156 * t2885;
    let t8148 = t496 * t8146 / 3.0;
    let t8149 = t2890 * t1243;
    let t8151 = t128 * t8102;
    let t8152 = t10 * t8151;
    let t8155 = -t5749 - t5751 + t5753 - t5755 - t5759 - 0.97936000000000000001e0 * t5764 + 0.73452e0 * t5768 + t8117 - t5776 - 6.0 * t496 * t10 * t8118 - t8126 + 3.0 * t496 * t10 * t8127 + 3.0 / 2.0 * t496 * t10 * t8131 - t8137 - t8139 + t8142 + t8145 + t8148 - 0.97936e0 * t8149 - t496 * t8152 / 2.0;
    (t8142, t8145, t8146, t8152, t8155)
}
