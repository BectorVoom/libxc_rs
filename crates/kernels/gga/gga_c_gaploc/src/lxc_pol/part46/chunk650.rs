//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 650/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk650<F: Float>(t13199: F, t13228: F, t12847: F, t12849: F, t12853: F, t12855: F, t12858: F, t13002: F, t13004: F, t13005: F, t13006: F, t13166: F, t331: F, t748: F, t2592: F, t3511: F) -> (F, F, F) {
    let t13229 = t13199 + t13228;
    let t13231 = -t748 * t13166 + t13229 * t331 + t12847 - t12849 - t12853 + t12855 + t12858 + t13002 + t13004 - t13005 - 2.0 * t13006;
    let t13232 = t2592 * t3511;
    (t13229, t13231, t13232)
}
