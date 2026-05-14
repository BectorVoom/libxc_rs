//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 910/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk910<F: Float>(t35827: F, t30318: F, t537: F, t7433: F, t8908: F, t8912: F, t7346: F, t7347: F, t8480: F, t7447: F, t8823: F, t7440: F, t8826: F, t30817: F, t8948: F, t8793: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35828 = 0.14291339372689912324e-3 * t35827;
    let t35829 = t30318 * t537;
    let t35835 = t7433 * t8908;
    let t35836 = 0.25724410870841842184e-2 * t35835;
    let t35837 = t7433 * t8912;
    let t35838 = 0.12862205435420921092e-2 * t35837;
    let t35844 = t7346 * t8480 * t7347;
    let t35845 = 0.21437009059034868486e-3 * t35844;
    let t35848 = t7447 * t8823;
    let t35849 = 0.84046875e-1 * t35848;
    let t35850 = t7440 * t8826;
    let t35851 = 0.5603125e-1 * t35850;
    let t35874 = t30817 * t8948;
    let t35875 = 0.25724410870841842184e-2 * t35874;
    let t35876 = t30817 * t8793;
    (t35828, t35829, t35836, t35838, t35845, t35849, t35851, t35875, t35876)
}
