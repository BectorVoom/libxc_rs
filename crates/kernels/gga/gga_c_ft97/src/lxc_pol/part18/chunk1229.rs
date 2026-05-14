//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1229/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1229<F: Float>(t11837: F, t5743: F, t46727: F, t5731: F, t1852: F, t3255: F, t26113: F, t492: F, t1820: F, t6557: F, t1286: F, t22873: F, t23385: F, t25533: F, t25535: F, t25595: F, t28: F, t5495: F, t8418: F, t94021: F, t94024: F, t94026: F, t94033: F, t94036: F, t948: F) -> (F, F, F, F, F, F) {
    let t102401 = t11837 * t5743;
    let t102417 = t46727 * t5731;
    let t102420 = t1852 * t5743 * t3255;
    let t102423 = t1852 * t26113 * t492;
    let t102426 = t1852 * t6557 * t1820;
    let t102428 = 2.0 / 9.0 * t94021 - 4.0 * t102401 - t948 * t23385 - 24.0 * t8418 * t25595 * t492 - 2.0 / 3.0 * t1286 * t28 * t22873 * t25533 + 4.0 / 27.0 * t94024 - t94026 / 27.0 + t94033 / 27.0 - 4.0 / 81.0 * t94036 - 2.0 / 3.0 * t5495 * t25535 + 8.0 * t102417 + 8.0 * t102420 + 8.0 * t102423 + 4.0 * t102426;
    (t102401, t102417, t102420, t102423, t102426, t102428)
}
