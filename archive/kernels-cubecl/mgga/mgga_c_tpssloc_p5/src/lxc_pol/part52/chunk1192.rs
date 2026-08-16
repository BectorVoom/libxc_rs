//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1192/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1192<F: Float>(t31840: F, t31849: F, t31895: F, t31921: F, t3: F, t112: F, t8692: F, t1873: F, t24969: F, t24972: F, t7015: F, t6534: F, t7423: F) -> (F, F, F, F, F, F) {
    let t31923 = t31840 + t31849 + t31895 + t31921;
    let t31924 = t3 * t31923;
    let t31937 = t8692 * t112;
    let t31940 = t24969 * t1873;
    let t31942 = t24972 * t7015;
    let t31944 = t7423 * t6534;
    (t31923, t31924, t31937, t31940, t31942, t31944)
}
