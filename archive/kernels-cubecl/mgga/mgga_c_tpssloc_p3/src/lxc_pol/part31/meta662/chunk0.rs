//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1949/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1949<F: Float>(t28299: F, t81979: F, t28273: F, t6547: F, t28264: F, t17022: F, t1880: F, t214: F, t225: F, t258: F, t28272: F, t6562: F, t794: F) -> (F, F, F, F, F) {
    let t98993 = t81979 * t28299;
    let t98995 = t6547 * t28273;
    let t99003 = t6547 * t28264;
    let t99019 = t1880 * t214 * t17022 * t225 * t258;
    let t99022 = t6562 * t794 * t28272;
    (t98993, t98995, t99003, t99019, t99022)
}
