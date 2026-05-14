//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1038/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1038<F: Float>(t10265: F, t170: F, t595: F, t9903: F, t159: F, t2769: F, t3137: F, t5815: F, t5818: F, t5821: F, t5834: F, t5925: F, t5940: F, t5945: F, t5950: F, t5959: F, t5963: F, t5966: F, t5975: F, t598: F, t5985: F, t5987: F, t9015: F) -> (F, F, F) {
    let t10266 = t10265 * t170;
    let t10269 = t595 * t9903;
    let t10274 = t5815 + t5925 - 3.0 * t9015 - t5818 + t5821 + t5940 + t5945 - t5950 + t5959 + t5963 - t5966 - t5975 + 0.285764e-1 * t159 * t10266 + t5834 + t5985 + t5987 - 0.675260332e-1 * t10269 * t598 - 0.2025780996e0 * t3137 * t2769;
    (t10266, t10269, t10274)
}
