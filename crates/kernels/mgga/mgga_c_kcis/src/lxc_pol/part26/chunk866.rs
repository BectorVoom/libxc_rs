//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 866/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk866<F: Float>(t169: F, t174: F, t18431: F, t447: F, t113: F, t13003: F, t6272: F, t2629: F, t6276: F, t171: F, t2633: F, t4510: F, t829: F, t13014: F, t6281: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t18432 = piecewise3::<f64>(t175, F::new(0.0), t18431);
    let t18433 = t447 * t18432;
    let t18443 = -t18431;
    let t19653 = F::new(2.0) * t113;
    let t20828 = t13003 * t6272;
    let t20833 = t2629 * t6276;
    let t20839 = piecewise3::<f64>(t170, F::new(0.0), -F::new(8.0) / F::new(27.0) * t20828 * t829 + F::new(16.0) / F::new(9.0) * t4510 * t2633 + F::new(4.0) / F::new(9.0) * t20833 * t829 + F::new(4.0) / F::new(3.0) * t171 * t18443);
    let t20840 = t13014 * t6281;
    (t18432, t18433, t19653, t20839, t20840)
}
