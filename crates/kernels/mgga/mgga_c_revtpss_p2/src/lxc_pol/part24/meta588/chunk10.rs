//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1846/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1846<F: Float>(t198: F, t3828: F, t40076: F, t40079: F, t47122: F, t47124: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F, t91875: F, t92024: F, t92026: F, t92027: F, t92028: F, t92029: F) -> F {
    let t92504 = F::new(18.0) * t198 * t3828 * t91875 + t40076 - t40079 + t47122 + t47124 + t47131 - t47138 - t47140 + t47142 + t47152 - t92024 + t92026 + t92027 + t92028 + t92029;
    t92504
}
