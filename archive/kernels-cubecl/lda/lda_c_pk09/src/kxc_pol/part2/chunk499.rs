//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 499/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk499<F: Float>(t2811: F, t444: F, t2036: F, t2759: F, t477: F, t2061: F, t2063: F, t2065: F, t2067: F, t2733: F, t2736: F, t2803: F, t2807: F) -> (F, F, F, F) {
    let t2812 = t2811 * t444;
    let t2813 = t2812 * t2036;
    let t2817 = t2759 * t477;
    let t2824 = t2061 - F::cast_from(1.5323028051206833_f64) * t2803 + t2063 + F::cast_from(1.5323028051206833_f64) * t2807 + t2065 - F::cast_from(0.3056501876701794_f64) * t2733 + t2067 + F::cast_from(0.3056501876701794_f64) * t2736;
    (t2812, t2813, t2817, t2824)
}
