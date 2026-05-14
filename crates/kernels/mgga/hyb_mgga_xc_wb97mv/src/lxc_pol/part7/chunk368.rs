//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 368/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk368<F: Float>(t1330: F, t788: F, t795: F, t1323: F, t226: F, t238: F, t242: F, t1325: F, t793: F, t803: F) -> (F, F, F, F, F) {
    let t1331 = t788 * t1330;
    let t1334 = t795 * t1330;
    let t1336 = t226 * t1323;
    let t1338 = t238 * t242 * t1336;
    let t1340 = 0.1898925e1 * t1331 - t793 + 0.8969e0 * t1325 + 0.3071625e0 * t1334 - t803 + 0.24647e0 * t1338;
    (t1331, t1334, t1336, t1338, t1340)
}
