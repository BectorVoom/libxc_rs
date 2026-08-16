//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2950/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2950(t13822: f64, t17757: f64, t973: f64, t17772: f64, t2970: f64, t13931: f64, t17773: f64, t17841: f64, t2960: f64, t343: f64, t4546: f64, t48292: f64, t48297: f64, t48302: f64, t48317: f64, t48320: f64, t48328: f64, t55677: f64, t7577: f64, t977: f64, t978: f64, t984: f64) -> f64 {
    let t61427 = t973 * t13822 * t17757;
    let t61447 = t973 * t2970 * t17772;
    let t61453 = -0.55555555555555555554e-3_f64 * t61427 - 0.16666666666666666666e-2_f64 * t973 * t4546 * t17841 * t984 * t343 + 0.74074074074074074072e-3_f64 * t48292 + 0.29629629629629629628e-2_f64 * t48297 + 0.18518518518518518518e-3_f64 * t48302 - 0.98765432098765432096e-3_f64 * t48317 - 0.24691358024691358024e-3_f64 * t48320 + 0.6584362139917695473e-3_f64 * t48328 - 0.16666666666666666666e-2_f64 * t973 * t4546 * t7577 * t13931 - 0.14814814814814814814e-2_f64 * t2960 * t17773 + 0.18518518518518518518e-3_f64 * t61447 + 0.27777777777777777777e-3_f64 * t973 * t977 * t978 * t55677;
    t61453
}
