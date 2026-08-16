//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2009/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2009(t90551: f64, t90582: f64, t90584: f64, t16122: f64, t1843: f64, t2085: f64, t24095: f64, t26996: f64, t27062: f64, t27068: f64, t3758: f64, t3882: f64, t3912: f64, t5354: f64, t568: f64, t80711: f64, t84655: f64, t90594: f64, t90598: f64) -> (f64, f64) {
    let t93368 = 0.10417915756705434098e0_f64 * t90551;
    let t93387 = 0.52089578783527170489e-1_f64 * t90582;
    let t93388 = 0.15352717957250113407e0_f64 * t90584;
    let t93399 = 4.0_f64 * t3882 * t26996 - 0.10417915756705434098e0_f64 * t80711 + t93387 + t93388 + 4.0_f64 * t3758 * t27062 - t84655 * t1843 - 2.0_f64 * t24095 * t5354 + t16122 * t2085 * t568 - 0.39478417604357434476e0_f64 * t90594 - t27068 * t3912 - 0.3289868133696452873e-1_f64 * t90598;
    (t93368, t93399)
}
