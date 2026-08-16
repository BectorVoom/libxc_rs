//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 583/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk583<F: Float>(t4256: F, t4257: F, t2059: F, t4254: F, t142: F, t435: F) -> (F, F, F) {
    let t4258 = t4256 * t4257;
    let t4261 = t4254 * t2059;
    let t4262 = t142 * t435;
    (t4258, t4261, t4262)
}
