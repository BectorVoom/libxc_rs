//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1033/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1033<F: Float>(t195: F, t8708: F, t3359: F, t642: F, t1062: F, t2531: F, t2724: F, t998: F, t3507: F, t462: F, t4872: F, t3380: F, t46: F, t552: F, t6804: F, t3363: F, t5093: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8709 = t8708 * t195;
    let t8710 = t3359 * t642;
    let t8711 = t2531 * t1062;
    let t8713 = t998 * t2724;
    let t8715 = t462 * t3507;
    let t8716 = 0.10843581300301739842e-1 * t4872;
    let t8717 = t3380 * t46;
    let t8718 = t8717 * t552;
    let t8719 = 0.18311447306006545054e-3 * t8718;
    let t8720 = 0.48830526149350786811e-3 * t6804;
    let t8721 = t5093 * t3363;
    (t8709, t8710, t8711, t8713, t8715, t8716, t8717, t8719, t8720, t8721)
}
