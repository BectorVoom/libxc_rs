//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2007;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2008;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2009;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta634(t90500: f64, t90503: f64, t90511: f64, t225: f64, t27070: f64, t27052: f64, t90514: f64, t90524: f64, t90533: f64, t90541: f64, t90546: f64, t90549: f64, t1375: f64, t16436: f64, t2091: f64, t3887: f64, t80689: f64, t90521: f64, t90527: f64, t90530: f64, t90539: f64, t90551: f64, t90582: f64, t90584: f64, t16122: f64, t1843: f64, t2085: f64, t24095: f64, t26996: f64, t27062: f64, t27068: f64, t3758: f64, t3882: f64, t3912: f64, t5354: f64, t568: f64, t80711: f64, t84655: f64, t90594: f64, t90598: f64, t90604: f64, t90609: f64, t16030: f64, t24082: f64, t24088: f64, t24147: f64, t5215: f64, t5321: f64, t5326: f64, t7199: f64, t80738: f64, t84400: f64, t90626: f64, t90634: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93333, t93335, t93337, t93338, t93341, t93344, t93350, t93353, t93359, t93361, t93362) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2007(t90500, t90503, t90511, t225, t27070, t27052, t90514, t90524, t90533, t90541, t90546, t90549);
        let t93363 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2008(t1375, t16436, t2091, t3887, t80689, t90521, t90527, t90530, t90539, t93350, t93353, t93359, t93361, t93362);
        let (t93368, t93399) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2009(t90551, t90582, t90584, t16122, t1843, t2085, t24095, t26996, t27062, t27068, t3758, t3882, t3912, t5354, t568, t80711, t84655, t90594, t90598);
        let (t93404, t93407, t93431) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2010(t90604, t90609, t16030, t24082, t24088, t24095, t24147, t26996, t3758, t5215, t5321, t5326, t7199, t80738, t84400, t90626, t90634);
    (t93333, t93335, t93337, t93338, t93341, t93344, t93363, t93368, t93399, t93404, t93407, t93431)
}
