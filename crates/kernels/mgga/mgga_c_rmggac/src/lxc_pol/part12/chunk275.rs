//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 275/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk275<F: Float>(t53: F, t333: F, t559: F, t338: F, t558: F, t352: F, t171: F, t577: F, t433: F, t521: F, t983: F, t437: F, t50: F, t280: F, t814: F, t525: F, t990: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t1365 = t559 * t333;
    let t1368 = t338 * t558;
    let t1369 = t1368 * t352;
    let t1372 = t577 * t171;
    let t1373 = t1372 * t433;
    let t1374 = 0.5848223622634646207e0 * t1373;
    let t1375 = t983 * t521;
    let t1378 = t437 * t50;
    let t1382 = piecewise3(t54, 0.0, -2.0 / 9.0 * t1375 * t280 + 4.0 / 3.0 * t1378 * t814);
    let t1383 = t990 * t525;
    (t1365, t1368, t1369, t1372, t1374, t1375, t1378, t1382, t1383)
}
