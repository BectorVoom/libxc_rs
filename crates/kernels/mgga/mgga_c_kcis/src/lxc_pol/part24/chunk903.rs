//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 903/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk903<F: Float>(t169: F, t174: F, t171: F, t18443: F, t20828: F, t20833: F, t2633: F, t4510: F, t829: F, t13014: F, t6281: F, t2641: F, t6284: F, t176: F, t18431: F, t4518: F, t833: F, t44: F, zeta_threshold: F) -> (F,) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t20839 = piecewise3(t170, 0.0, -8.0 / 27.0 * t20828 * t829 + 16.0 / 9.0 * t4510 * t2633 + 4.0 / 9.0 * t20833 * t829 + 4.0 / 3.0 * t171 * t18443);
    let t20840 = t13014 * t6281;
    let t20845 = t2641 * t6284;
    let t20851 = piecewise3(t175, 0.0, -8.0 / 27.0 * t20840 * t833 - 16.0 / 9.0 * t4518 * t2633 + 4.0 / 9.0 * t20845 * t833 + 4.0 / 3.0 * t176 * t18431);
    let t20853 = (t20839 + t20851) * t44;
    (t20853,)
}
