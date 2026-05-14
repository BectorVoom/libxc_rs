//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 712/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk712<F: Float>(t13854: F, t13887: F, t13903: F, t13912: F, t12846: F, t12850: F, t12851: F, t12853: F, t12854: F, t13004: F, t13006: F, t13243: F, t13761: F, t13762: F, t13767: F, t13839: F, t13841: F, t748: F) -> (F, F) {
    let t13914 = t13854 + t13887 + t13903 + t13912;
    let t13916 = -t13914 * t748 + t12846 + t12850 + t12851 - t12853 + t12854 + t13004 - t13006 + t13243 - t13761 + t13762 - t13767 + 2.0 * t13839 - t13841;
    (t13914, t13916)
}
