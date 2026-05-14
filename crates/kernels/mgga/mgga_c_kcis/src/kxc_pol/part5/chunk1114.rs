//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1114/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1114<F: Float>(t169: F, t1876: F, t4534: F, t233: F, t1881: F, t5411: F, t13003: F, t6272: F, t2629: F, t6276: F, t171: F, t18443: F, t2633: F, t4510: F, t829: F, t13014: F, t6281: F, zeta_threshold: F) -> (F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t20823 = t4534 * t1876;
    let t20824 = t233 * t20823;
    let t20826 = t1881 * t5411;
    let t20828 = t13003 * t6272;
    let t20833 = t2629 * t6276;
    let t20839 = piecewise3(t170, 0.0, -8.0 / 27.0 * t20828 * t829 + 16.0 / 9.0 * t4510 * t2633 + 4.0 / 9.0 * t20833 * t829 + 4.0 / 3.0 * t171 * t18443);
    let t20840 = t13014 * t6281;
    (t20824, t20826, t20839, t20840)
}
