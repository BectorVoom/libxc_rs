//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 283/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk283<F: Float>(t333: F, t559: F, t338: F, t558: F, t352: F, t171: F, t577: F, t433: F, t521: F, t983: F, t437: F, t50: F) -> (F, F, F, F, F, F, F) {
    let t1365 = t559 * t333;
    let t1368 = t338 * t558;
    let t1369 = t1368 * t352;
    let t1372 = t577 * t171;
    let t1373 = t1372 * t433;
    let t1374 = F::new(0.5848223622634646207e0) * t1373;
    let t1375 = t983 * t521;
    let t1378 = t437 * t50;
    (t1365, t1368, t1369, t1372, t1374, t1375, t1378)
}
