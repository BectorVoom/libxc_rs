//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1083/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1083<F: Float>(t11374: F, t995: F, t4327: F, t967: F, t11267: F, t11282: F, t7190: F, t7192: F, t9271: F, t9275: F, t359: F, t7221: F, t9409: F, t4332: F, t7362: F, t975: F) -> (F, F, F, F, F, F, F) {
    let t11395 = t11374 * t995;
    let t11400 = t4327 * t967;
    let t11414 = -t7190 + 0.23744444444444444444e-1 * t7192 + 0.47488888888888888888e-1 * t9271 - t9275 - 0.17808333333333333333e-1 * t11267 + 0.53425e-1 * t11282;
    let t11416 = 0.621814e-1 * t11414 * t359;
    let t11421 = -t7221 + 0.22831111111111111111e-1 * t7192 + 0.45662222222222222221e-1 * t9271 - t9409 - 0.17123333333333333333e-1 * t11267 + 0.5137e-1 * t11282;
    let t11424 = t4332 * t7362;
    let t11425 = t11424 * t975;
    (t11395, t11400, t11414, t11416, t11421, t11424, t11425)
}
