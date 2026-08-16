//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1984/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1984<F: Float>(t1388: F, t6330: F, t6463: F, t1307: F, t5449: F, t671: F, t1851: F, t1372: F, t794: F, t213: F, t225: F, t22716: F, t6908: F) -> (F, F, F, F, F, F, F, F) {
    let t75203 = t6330 * t1388;
    let t75210 = t6463 * t1388;
    let t75214 = t6463 * t1307;
    let t75560 = t5449 * t671;
    let t75795 = t1851 * t671;
    let t80645 = t794 * t1372;
    let t80650 = t213 * t1372 * t225;
    let t80663 = t22716 * t6908;
    (t75203, t75210, t75214, t75560, t75795, t80645, t80650, t80663)
}
