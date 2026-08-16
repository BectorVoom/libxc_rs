//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 756/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk756<F: Float>(t104: F, t2248: F, t3984: F, t8040: F, t2217: F, t322: F, t2132: F, t2138: F, t633: F, t879: F, t2147: F, t463: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8048 = t104 * t2248;
    let t8052 = t8040 * t3984;
    let t8060 = t2217 * t322;
    let t8061 = t2132 * t8060;
    let t8062 = t2138 * t8061;
    let t8064 = t633 * t879;
    let t8065 = t2132 * t8064;
    let t8067 = F::cast_from(0.8673628188205199462e0_f64) * t2138 * t8065;
    let t8069 = t2147 * t2217 * t463;
    (t8048, t8052, t8060, t8061, t8062, t8064, t8065, t8067, t8069)
}
