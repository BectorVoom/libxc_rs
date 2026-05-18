//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 989/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk989<F: Float>(t2132: F, t2138: F, t322: F, t8301: F, t2230: F, t29985: F, t7987: F, t8104: F, t2131: F, t3644: F, t633: F, t2217: F, t879: F) -> (F, F, F, F, F) {
    let t33262 = t2138 * t2132 * t8301 * t322;
    let t33264 = t29985 * t2230;
    let t33266 = t7987 * t8104;
    let t33271 = F::new(0.8673628188205199462e0) * t2131 * t2132 * t633 * t3644;
    let t33274 = t2138 * t2132 * t2217 * t879;
    (t33262, t33264, t33266, t33271, t33274)
}
