//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2009/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2009<F: Float>(t90551: F, t90582: F, t90584: F, t16122: F, t1843: F, t2085: F, t24095: F, t26996: F, t27062: F, t27068: F, t3758: F, t3882: F, t3912: F, t5354: F, t568: F, t80711: F, t84655: F, t90594: F, t90598: F) -> (F, F) {
    let t93368 = F::cast_from(0.10417915756705434098e0_f64) * t90551;
    let t93387 = F::cast_from(0.52089578783527170489e-1_f64) * t90582;
    let t93388 = F::cast_from(0.15352717957250113407e0_f64) * t90584;
    let t93399 = F::cast_from(4.0_f64) * t3882 * t26996 - F::cast_from(0.10417915756705434098e0_f64) * t80711 + t93387 + t93388 + F::cast_from(4.0_f64) * t3758 * t27062 - t84655 * t1843 - F::cast_from(2.0_f64) * t24095 * t5354 + t16122 * t2085 * t568 - F::cast_from(0.39478417604357434476e0_f64) * t90594 - t27068 * t3912 - F::cast_from(0.3289868133696452873e-1_f64) * t90598;
    (t93368, t93399)
}
