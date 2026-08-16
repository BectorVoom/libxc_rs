//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1039;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1040;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1041;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta305(t1055: f64, t21662: f64, t1603: f64, t5914: f64, t1634: f64, t5919: f64, t10165: f64, t21480: f64, t381: f64, t1625: f64, t5848: f64, t21614: f64, t349: f64, t5943: f64, t3174: f64, t1052: f64, t1635: f64, t17575: f64, t17588: f64, t18074: f64, t388: f64, t4557: f64, t4660: f64, t5920: f64, t5944: f64, t1070: f64, t193: f64, t21251: f64, t21255: f64, t21263: f64, t21265: f64, t21267: f64, t21270: f64, t21302: f64, t21305: f64, t21317: f64, t21320: f64, t21336: f64, t21591: f64, t336: f64, t25: f64, t265: f64, t394: f64, t21076: f64, t21381: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t20216: f64, t20217: f64, t396: f64, t40: f64, t5397: f64, t5398: f64, t5669: f64, t5955: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21663, t21669, t21677, t21682, t21684, t21689) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1039(t1055, t21662, t1603, t5914, t1634, t5919, t10165, t21480, t381, t1625, t5848, t21614, t349);
        let (t21692, t21697) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1040(t1634, t5943, t3174, t1052, t1635, t17575, t17588, t18074, t21663, t21669, t21677, t21682, t21684, t21689, t388, t4557, t4660, t5920, t5944);
        let t21701 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1041(t1070, t193, t21251, t21255, t21263, t21265, t21267, t21270, t21302, t21305, t21317, t21320, t21336, t21591, t21697, t336);
        let (t21703, t21713) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1042(t25, t265, t394, t21076, t21381, t21701, t1408, t1409, t1534, t1642, t20216, t20217, t396, t40, t5397, t5398, t5669, t5955, dens_threshold, rho0, zeta_threshold);
    (t21663, t21669, t21677, t21682, t21684, t21689, t21692, t21697, t21703, t21713)
}
