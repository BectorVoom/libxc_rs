//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1006/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1006<F: Float>(t1395: F, t2406: F, t2425: F, t220: F, t73: F, t8275: F, t1378: F, t806: F, t246: F, t3664: F, t1388: F, t2157: F) -> (F, F, F, F, F) {
    let t10841 = t2406 * t1395 * t2425;
    let t10845 = t220 * t73 * t8275;
    let t10849 = t806 * t1378;
    let t10853 = t246 * t3664;
    let t10880 = t2157 * t1388;
    (t10841, t10845, t10849, t10853, t10880)
}
