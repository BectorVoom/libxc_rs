//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2007;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2008;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2009;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta634<F: Float>(t90500: F, t90503: F, t90511: F, t225: F, t27070: F, t27052: F, t90514: F, t90524: F, t90533: F, t90541: F, t90546: F, t90549: F, t1375: F, t16436: F, t2091: F, t3887: F, t80689: F, t90521: F, t90527: F, t90530: F, t90539: F, t90551: F, t90582: F, t90584: F, t16122: F, t1843: F, t2085: F, t24095: F, t26996: F, t27062: F, t27068: F, t3758: F, t3882: F, t3912: F, t5354: F, t568: F, t80711: F, t84655: F, t90594: F, t90598: F, t90604: F, t90609: F, t16030: F, t24082: F, t24088: F, t24147: F, t5215: F, t5321: F, t5326: F, t7199: F, t80738: F, t84400: F, t90626: F, t90634: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93333, t93335, t93337, t93338, t93341, t93344, t93350, t93353, t93359, t93361, t93362) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2007::<F>(t90500, t90503, t90511, t225, t27070, t27052, t90514, t90524, t90533, t90541, t90546, t90549);
        let t93363 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2008::<F>(t1375, t16436, t2091, t3887, t80689, t90521, t90527, t90530, t90539, t93350, t93353, t93359, t93361, t93362);
        let (t93368, t93399) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2009::<F>(t90551, t90582, t90584, t16122, t1843, t2085, t24095, t26996, t27062, t27068, t3758, t3882, t3912, t5354, t568, t80711, t84655, t90594, t90598);
        let (t93404, t93407, t93431) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2010::<F>(t90604, t90609, t16030, t24082, t24088, t24095, t24147, t26996, t3758, t5215, t5321, t5326, t7199, t80738, t84400, t90626, t90634);
    (t93333, t93335, t93337, t93338, t93341, t93344, t93363, t93368, t93399, t93404, t93407, t93431)
}
