//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 816/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk816<F: Float>(t195: F, t8708: F, t3359: F, t642: F, t1062: F, t2531: F, t2724: F, t998: F, t3507: F, t462: F, t4872: F, t3380: F, t46: F) -> (F, F, F, F, F, F, F) {
    let t8709 = t8708 * t195;
    let t8710 = t3359 * t642;
    let t8711 = t2531 * t1062;
    let t8713 = t998 * t2724;
    let t8715 = t462 * t3507;
    let t8716 = F::cast_from(0.10843581300301739842e-1_f64) * t4872;
    let t8717 = t3380 * t46;
    (t8709, t8710, t8711, t8713, t8715, t8716, t8717)
}
