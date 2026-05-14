//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1039/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1039<F: Float>(t10207: F, t10217: F, t10227: F, t10237: F, t10244: F, t10254: F, t10263: F, t10274: F, t10265: F, t246: F, t4703: F, t4721: F, t4880: F, t4882: F, t4887: F, t4891: F, t4897: F, t4901: F, t4964: F, t4967: F, t9034: F, t9036: F, t9040: F, t9884: F, t9885: F) -> (F, F) {
    let t10277 = t10207 + t10217 + t10227 + t10237 + t10244 + t10254 + t10263 + t10274;
    let t10286 = -t4880 - t4882 - 0.2025780996e0 * t9034 - 0.4051561992e0 * t9036 + t4887 + t4891 - t4703 - t4897 + t9884 - t4901 + t9885 + 0.857292e-1 * t9040 - t4721 - 0.285764e-1 * t246 * t10265 + t4964 - t4967;
    (t10277, t10286)
}
