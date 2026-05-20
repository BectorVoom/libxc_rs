//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3096/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3096<F: Float>(t3588: F, t5341: F, t12904: F, t5293: F, t12959: F, t17569: F, t11262: F, t1261: F, t5269: F, t17236: F, t3172: F, t17540: F, t3711: F) -> (F, F, F, F, F, F) {
    let t56766 = t5341 * t3588;
    let t56785 = t5293 * t12904;
    let t56786 = F::cast_from(0.7622047665434619906e-3_f64) * t56785;
    let t56787 = t17569 * t12959;
    let t56790 = t1261 * t11262 * t5269;
    let t56791 = F::cast_from(0.19055119163586549765e-3_f64) * t56790;
    let t56793 = t1261 * t3172 * t17236;
    let t56796 = t3711 * t3172 * t17540;
    (t56766, t56786, t56787, t56791, t56793, t56796)
}
