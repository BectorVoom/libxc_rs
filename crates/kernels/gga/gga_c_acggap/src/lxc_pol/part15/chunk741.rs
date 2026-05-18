//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 741/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk741<F: Float>(t104: F, t2248: F, t2217: F, t322: F, t2132: F, t2138: F, t633: F, t879: F, t2147: F, t2225: F, t463: F, t2131: F) -> (F, F, F, F, F, F, F, F) {
    let t8048 = t104 * t2248;
    let t8060 = t2217 * t322;
    let t8061 = t2132 * t8060;
    let t8062 = t2138 * t8061;
    let t8064 = t633 * t879;
    let t8065 = t2132 * t8064;
    let t8067 = F::new(0.8673628188205199462e0) * t2138 * t8065;
    let t8073 = t2147 * t2225 * t463;
    let t8074 = t2131 * t8073;
    (t8048, t8061, t8062, t8064, t8065, t8067, t8073, t8074)
}
