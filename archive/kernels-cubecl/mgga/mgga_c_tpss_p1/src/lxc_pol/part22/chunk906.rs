//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 906/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk906<F: Float>(t2376: F, t339: F, t795: F, t803: F, t2383: F, t2395: F, t207: F, t237: F, t235: F, t72: F, t2146: F, t756: F) -> (F, F, F, F, F) {
    let t8130 = t339 * t795 * t2376;
    let t8131 = t8130 * t803;
    let t8133 = t2383 * t2395;
    let t8160 = F::cast_from(1.0_f64) / t237 / t207;
    let t8162 = t235 * t8160 * t72;
    let t8167 = t756 * t2146;
    (t8130, t8131, t8133, t8162, t8167)
}
