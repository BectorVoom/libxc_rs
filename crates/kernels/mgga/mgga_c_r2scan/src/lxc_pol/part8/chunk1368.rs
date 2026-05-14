//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1368/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1368<F: Float>(t1859: F, t1862: F, t9903: F, t21221: F, t21225: F, t21228: F, t21232: F, t21237: F, t21244: F, t21247: F, t21251: F, t21254: F, t26436: F, t26438: F, t2774: F, t28910: F, t8893: F, t951: F) -> (F,) {
    let t33451 = t1859 * t9903 * t1862;
    let t33459 = -t21221 - 0.12154685976e1 * t21225 + 0.30762056574649219974e4 * t21228 + t21232 + t21237 - t21244 + 0.1350520664e0 * t33451 + t21247 - 0.2025780996e0 * t2774 * t8893 - 0.2025780996e0 * t951 * t28910 - 0.85556969848243143048e2 * t26436 + 0.97592231702715658578e-1 * t26438 + t21251 - t21254;
    (t33459,)
}
