//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1640;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1641;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta449(t5: f64, t24006: f64, t112: f64, t1268: f64, t12734: f64, t12739: f64, t2039: f64, t2314: f64, t2363: f64, t23917: f64, t23938: f64, t23941: f64, t5113: f64, t671: f64, t7042: f64, t7056: f64, t9348: f64, t6999: f64, t7217: f64, t22754: f64, t22757: f64, t22762: f64, t22766: f64, t22768: f64, t22771: f64, t22774: f64, t22777: f64, t22780: f64, t22784: f64, t22786: f64, t22789: f64, t22795: f64, t22798: f64, t22800: f64, t22819: f64, t22825: f64, t22858: f64, t22863: f64, t22867: f64, t22805: f64, t22809: f64, t22830: f64, t22834: f64, t22837: f64, t22840: f64, t22848: f64, t22850: f64, t22856: f64, t22860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24007, t24008, t24026) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1640(t5, t24006, t112, t1268, t12734, t12739, t2039, t2314, t2363, t23917, t23938, t23941, t5113, t671, t7042, t7056, t9348);
        let (t24028, t24046) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1641(t6999, t7217, t22754, t22757, t22762, t22766, t22768, t22771, t22774, t22777, t22780, t22784, t22786, t22789, t22795, t22798, t22800);
        let (t24049, t24050, t24058, t24060, t24061, t24062) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1642(t22819, t22825, t22858, t22863, t22867, t22805, t22809, t22830, t22834, t22837, t22840, t22848, t22850, t22856, t22860);
    (t24007, t24008, t24026, t24028, t24046, t24049, t24050, t24058, t24060, t24061, t24062)
}
