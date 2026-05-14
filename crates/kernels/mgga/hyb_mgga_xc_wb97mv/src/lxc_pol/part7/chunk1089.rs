//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1089/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1089<F: Float>(t11297: F, t11299: F, t11304: F, t11308: F, t11311: F, t11315: F, t11319: F, t7294: F, t7348: F, t9360: F, t9475: F, t9476: F, t11520: F, t976: F, t11477: F, t11480: F, t11483: F, t11486: F, t11489: F, t11493: F, t11496: F, t11500: F, t11502: F, t11505: F, t2529: F, t2556: F, t4346: F, t4349: F, t7328: F, t968: F) -> (F, F, F) {
    let t11530 = 0.31558125e0 * t11297 + 0.6311625e0 * t11299 - t7348 + 0.34731666666666666666e0 * t7294 + 0.69463333333333333333e0 * t9360 - t9475 - t9476 - 0.20839e0 * t11304 + 0.62517e0 * t11308 - 0.20839e0 * t11311 + 0.312585e0 * t11315 + 0.312585e0 * t11319;
    let t11531 = t11520 + t11530;
    let t11532 = t11531 * t976;
    let t11537 = -t11477 - t11480 + t11483 + t11486 + t11489 - t11493 - t11496 - t11500 + 0.32163958997385070134e2 * t2556 * t11502 + 0.64327917994770140268e2 * t2556 * t11505 + 1.0 * t2529 * t4346 + 1.0 * t968 * t11532 + 0.32163958997385070134e2 * t7328 * t4349;
    (t11531, t11532, t11537)
}
