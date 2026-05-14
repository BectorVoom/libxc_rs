//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 756/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk756<F: Float>(t169: F, t2628: F, t174: F, t2640: F, t1646: F, t167: F, t2629: F, t160: F, t171: F, t2630: F, t2635: F, t4510: F, t4513: F, t740: F, t829: F, t1650: F, zeta_threshold: F) -> (F, F) {
    let t170 = t169 <= zeta_threshold;
    let t13003 = 1.0 / t2628 / t169;
    let t13014 = 1.0 / t2640 / t174;
    let t13062 = t13003 * t1646;
    let t13065 = t2629 * t167;
    let t13076 = piecewise3(t170, 0.0, -8.0 / 27.0 * t13062 * t2630 + 16.0 / 9.0 * t13065 * t740 * t829 + 4.0 / 9.0 * t4510 * t2635 + 8.0 / 3.0 * t171 * t740 - 8.0 * t4513 * t160);
    let t13077 = t13014 * t1650;
    (t13076, t13077)
}
