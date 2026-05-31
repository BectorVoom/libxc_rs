//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 731/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk731<F: Float>(t154: F, t5688: F, t655: F, t276: F, t2003: F, t300: F, t2023: F, t768: F, t46: F, t2037: F, t747: F) -> (F, F, F, F, F, F, F) {
    let t5690 = t154 * t5688 * t655;
    let t5691 = t276 * t5690;
    let t5693 = t300 * t2003;
    let t5711 = t768 * t2023;
    let t5712 = t5711 * t46;
    let t5713 = t2037 * t5712;
    let t5716 = t747 * t747;
    let t5717 = F::cast_from(1.0_f64) / t5716;
    (t5691, t5693, t5711, t5712, t5713, t5716, t5717)
}
