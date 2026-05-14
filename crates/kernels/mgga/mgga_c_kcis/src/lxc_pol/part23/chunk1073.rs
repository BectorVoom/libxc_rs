//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1073/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1073<F: Float>(t27591: F, t27607: F, t2257: F, t2259: F, t44682: F, t1628: F, t27671: F, t27733: F, t26656: F, t13093: F, t2167: F, t4527: F, t7671: F, t93826: F, t1655: F, t26654: F) -> (F, F, F, F, F, F, F, F, F) {
    let t95157 = t27607 * t27591;
    let t95168 = 0.12871334876543209877e-3 * t2257 * t44682 * t2259;
    let t95235 = t27671 * t1628;
    let t95271 = 2.0 * t27733;
    let t95275 = 4.0 * t26656;
    let t97548 = t13093 * t2167;
    let t97561 = 2.0 * t4527 * t7671;
    let t97584 = 2.0 * t93826;
    let t97601 = t1655 * t26654;
    (t95157, t95168, t95235, t95271, t95275, t97548, t97561, t97584, t97601)
}
