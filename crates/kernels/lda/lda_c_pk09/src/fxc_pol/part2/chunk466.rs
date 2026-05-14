//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 466/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk466<F: Float>(t2811: F, t444: F, t2036: F, t2759: F, t477: F, t2061: F, t2063: F, t2065: F, t2067: F, t2733: F, t2736: F, t2803: F, t2807: F, t467: F, t452: F, t2758: F, t471: F) -> (F, F, F, F, F, F, F) {
    let t2812 = t2811 * t444;
    let t2813 = t2812 * t2036;
    let t2817 = t2759 * t477;
    let t2824 = t2061 - 1.5323028051206833 * t2803 + t2063 + 1.5323028051206833 * t2807 + t2065 - 0.3056501876701794 * t2733 + t2067 + 0.3056501876701794 * t2736;
    let t2825 = t467 * t2824;
    let t2826 = t2825 * t452;
    let t2829 = t471 * t2758;
    (t2812, t2813, t2817, t2824, t2825, t2826, t2829)
}
