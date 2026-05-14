//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1256/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1256<F: Float>(t322: F, t352: F, t6746: F, t23519: F, t1343: F, t2438: F, t1035: F, t1338: F, t1348: F, t19305: F, t19309: F, t19327: F, t2437: F, t2441: F, t2445: F, t6751: F, t6755: F, t6767: F, t8481: F, t8484: F, t8487: F, t8492: F, t8496: F, t8501: F, t855: F) -> (F,) {
    let t332 = 0.25e1 < t322;
    let t23639 = t352 * t6746;
    let t23644 = piecewise3(t332, t23519, 0.0);
    let t23648 = t2438 * t1343;
    let t23681 = -0.189e2 * t8496 * t8481 - 0.252e2 * t8487 * t23639 - 0.567e2 * t8501 * t23639 - 0.105e1 * t855 * t23644 * t352 - 0.189e2 * t2445 * t23648 - 0.2835e2 * t8487 * t23648 - 0.70875e1 * t8501 * t23648 - 0.63e1 * t1338 * t8492 * t2438 - 0.63e1 * t8484 * t6751 - 0.2835e2 * t6755 * t2441 * t8481 - 0.2835e2 * t19309 * t1035 * t23639 - 0.21e1 * t2437 * t19305 - 0.4725e1 * t1348 * t8492 * t2438 - 0.4725e1 * t8496 * t6751 - 0.70875e1 * t6767 * t2441 * t8481 - 0.1575e1 * t2445 * t19305 - 0.354375e1 * t19327 * t1035 * t23639;
    (t23681,)
}
