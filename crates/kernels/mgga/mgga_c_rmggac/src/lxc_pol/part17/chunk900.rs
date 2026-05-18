//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 900/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk900<F: Float>(t1970: F, t1971: F, t333: F, t511: F, t6172: F, t6349: F, t2136: F, t2186: F, t9938: F, t2191: F, t9731: F, t1986: F, t6602: F, t675: F) -> (F, F, F, F, F) {
    let t45080 = t1970 * t1971 * t511 * t6172 * t333;
    let t45086 = t6349 * t511;
    let t45087 = t45086 * t2136;
    let t45089 = t2186 * t9938;
    let t45091 = t2191 * t9731;
    let t45094 = t675 * t1986 * t6602;
    (t45080, t45087, t45089, t45091, t45094)
}
