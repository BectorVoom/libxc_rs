//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 757/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk757<F: Float>(t174: F, t167: F, t2641: F, t13077: F, t160: F, t176: F, t2642: F, t2645: F, t4518: F, t4521: F, t740: F, t833: F, t13076: F, t44: F, t251: F, t691: F, t102: F, t4880: F, zeta_threshold: F) -> (F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t13080 = t2641 * t167;
    let t13091 = piecewise3(t175, 0.0, -8.0 / 27.0 * t13077 * t2642 - 16.0 / 9.0 * t13080 * t740 * t833 + 4.0 / 9.0 * t4518 * t2645 - 8.0 / 3.0 * t176 * t740 + 8.0 * t4521 * t160);
    let t13093 = (t13076 + t13091) * t44;
    let t13396 = t691 * t251;
    let t13577 = t102 * t4880;
    (t13093, t13396, t13577)
}
