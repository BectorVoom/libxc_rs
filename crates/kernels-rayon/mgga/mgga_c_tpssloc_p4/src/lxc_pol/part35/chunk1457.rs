//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1457/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1457(t103218: f64, t103490: f64, t104504: f64, t104506: f64, t104509: f64, t104521: f64, t104527: f64, t1714: f64, t1716: f64, t21758: f64, t21769: f64, t22007: f64, t27406: f64, t27792: f64, t27799: f64, t29670: f64, t29813: f64, t6139: f64, t6244: f64, t7283: f64, t7285: f64, t7286: f64, t7300: f64, t8015: f64, t85674: f64, t85755: f64, t86451: f64, t95824: f64) -> f64 {
    let t109844 = -0.8529287754027840782e-2_f64 * t7283 * t85755 * t7286 * t21758 - 0.82246703342411321826e-2_f64 * t104504 - 0.16449340668482264365e-1_f64 * t7283 * t7285 * t7286 * t21769 + 0.14621636149762012769e-1_f64 * t104506 - 0.54831135561607547883e-2_f64 * t104509 - 0.24674011002723396548e-1_f64 * t7283 * t6139 * t1714 * t27799 + 0.82246703342411321826e-2_f64 * t104521 + 0.80418998823691070229e-1_f64 * t104527 + 6.0_f64 * t27792 * t6244 + 0.21932454224643019154e-1_f64 * t27406 * t29813 + t86451 - 0.49348022005446793095e-1_f64 * t7283 * t1716 * t103490 - 0.49348022005446793095e-1_f64 * t7283 * t7300 * t85674 * t22007 - 0.24674011002723396548e-1_f64 * t7283 * t1716 * t29670 + 0.14621636149762012769e-1_f64 * t95824 - 0.24125699647107321069e0_f64 * t103218 * t8015;
    t109844
}
