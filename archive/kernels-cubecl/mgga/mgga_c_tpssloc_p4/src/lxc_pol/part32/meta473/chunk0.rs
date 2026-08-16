//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1771/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1771<F: Float>(t1089: F, t491: F, t7327: F, t24574: F, t7365: F, t1235: F, t477: F, t225: F, t7349: F, t7288: F, t7306: F, t3640: F, t7394: F) -> (F, F, F, F, F, F, F, F) {
    let t24850 = t491 * t1089;
    let t24851 = t7327 * t24850;
    let t24856 = t24574 * t7365;
    let t24858 = t477 * t1235;
    let t24880 = t7349 * t225;
    let t24891 = t24574 * t7288;
    let t24893 = t7306 * t225;
    let t24905 = t7394 * t3640;
    (t24850, t24851, t24856, t24858, t24880, t24891, t24893, t24905)
}
