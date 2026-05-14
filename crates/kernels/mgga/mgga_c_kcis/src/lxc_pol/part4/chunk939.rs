//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 939/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk939<F: Float>(t169: F, t174: F, t13062: F, t13065: F, t160: F, t171: F, t2630: F, t2635: F, t4510: F, t4513: F, t740: F, t829: F, t13014: F, t1650: F, t167: F, t2641: F, t176: F, t2642: F, t2645: F, t4518: F, t4521: F, t833: F, zeta_threshold: F) -> (F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t13076 = piecewise3(t170, 0.0, -8.0 / 27.0 * t13062 * t2630 + 16.0 / 9.0 * t13065 * t740 * t829 + 4.0 / 9.0 * t4510 * t2635 + 8.0 / 3.0 * t171 * t740 - 8.0 * t4513 * t160);
    let t13077 = t13014 * t1650;
    let t13080 = t2641 * t167;
    let t13091 = piecewise3(t175, 0.0, -8.0 / 27.0 * t13077 * t2642 - 16.0 / 9.0 * t13080 * t740 * t833 + 4.0 / 9.0 * t4518 * t2645 - 8.0 / 3.0 * t176 * t740 + 8.0 * t4521 * t160);
    (t13076, t13091)
}
