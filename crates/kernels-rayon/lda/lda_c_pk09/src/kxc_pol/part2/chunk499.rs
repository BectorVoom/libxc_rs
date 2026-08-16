//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 499/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk499(t2811: f64, t444: f64, t2036: f64, t2759: f64, t477: f64, t2061: f64, t2063: f64, t2065: f64, t2067: f64, t2733: f64, t2736: f64, t2803: f64, t2807: f64) -> (f64, f64, f64, f64) {
    let t2812 = t2811 * t444;
    let t2813 = t2812 * t2036;
    let t2817 = t2759 * t477;
    let t2824 = t2061 - 1.5323028051206833_f64 * t2803 + t2063 + 1.5323028051206833_f64 * t2807 + t2065 - 0.3056501876701794_f64 * t2733 + t2067 + 0.3056501876701794_f64 * t2736;
    (t2812, t2813, t2817, t2824)
}
