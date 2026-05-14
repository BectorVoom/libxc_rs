//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 814/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk814<F: Float>(t10627: F, t10698: F, t828: F, t136: F, t2476: F, t221: F, t2394: F, t2674: F, t231: F, t243: F, t2645: F, t2662: F, t2661: F, t2652: F, t2656: F, t2482: F, t596: F, t849: F) -> (F, F, F, F, F, F, F) {
    let t10700 = t10698 * t828 * t10627;
    let t10703 = t2476 * t136;
    let t10705 = t10703 * t221 * t2394;
    let t10706 = t2674 * t10705;
    let t10709 = t243 * t2645 * t231;
    let t10710 = t2662 * t10709;
    let t10711 = t2661 * t10710;
    let t10713 = t2652 * t2656;
    let t10716 = t2482 * t849 * t596;
    (t10700, t10705, t10706, t10709, t10711, t10713, t10716)
}
