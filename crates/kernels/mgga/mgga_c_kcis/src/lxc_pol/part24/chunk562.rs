//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 562/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk562<F: Float>(t169: F, t174: F, t171: F, t2629: F, t6272: F, t6276: F, t1650: F, t176: F, t2641: F, t44: F, t234: F, t1709: F, t2811: F, t313: F, t1727: F, t4836: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t6280 = piecewise3(t170, 0.0, 4.0 / 9.0 * t2629 * t6272 + 4.0 / 3.0 * t171 * t6276);
    let t6281 = t1650 * t1650;
    let t6284 = -t6276;
    let t6288 = piecewise3(t175, 0.0, 4.0 / 9.0 * t2641 * t6281 + 4.0 / 3.0 * t176 * t6284);
    let t6290 = (t6280 + t6288) * t44;
    let t6293 = piecewise3(t170, 0.0, t6276);
    let t6294 = t234 * t6293;
    let t6301 = t1709 * t1709;
    let t6302 = t6301 * t2811;
    let t6307 = t313 * t6272;
    let t6310 = t4836 * t1727;
    let t6313 = t313 * t6276;
    let t6316 = t1727 * t1727;
    (t6281, t6284, t6290, t6293, t6294, t6301, t6302, t6307, t6310, t6313, t6316)
}
