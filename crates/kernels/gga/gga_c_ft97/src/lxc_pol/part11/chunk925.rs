//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 925/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk925<F: Float>(t41677: F, t807: F, t2426: F, t2428: F, t3724: F, t41448: F, t9577: F, t683: F, t92: F, t41482: F, t2360: F, t41468: F, t41490: F, t3051: F, t685: F, t1771: F, t2414: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t41682 = t807 * t41677;
    let t41686 = t3724 * t2426 * t2428;
    let t41691 = t9577 * t41448;
    let t41693 = t92 * t683 * t41691;
    let t41696 = t92 * t683 * t41482;
    let t41698 = t2360 * t41468;
    let t41700 = t92 * t683 * t41698;
    let t41703 = t92 * t683 * t41490;
    let t41705 = t3051 * t685;
    let t41707 = t1771 * t2414;
    (t41682, t41686, t41691, t41693, t41696, t41698, t41700, t41703, t41705, t41707)
}
