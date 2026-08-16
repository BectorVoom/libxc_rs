//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2406/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2406(t10828: f64, t1580: f64, t10524: f64, t10724: f64, t10740: f64, t10743: f64, t10771: f64, t10811: f64, t10825: f64, t14329: f64, t14425: f64, t14429: f64, t14435: f64, t14463: f64, t1581: f64, t2861: f64, t2862: f64, t2880: f64, t4434: f64, t4437: f64, t49222: f64, t49228: f64, t49244: f64, t49256: f64, t49259: f64, t49262: f64, t931: f64, t943: f64, t951: f64) -> f64 {
    let t49263 = t10828 * t1580;
    let t49266 = 0.5848223622634646207e0_f64 * t943 * t49222 * t951 - t49228 - 12.0_f64 * t10740 * t14425 - 6.0_f64 * t2861 * t14329 * t931 - 6.0_f64 * t2861 * t4434 * t2880 - 0.57895126195293126242e3_f64 * t10771 * t14435 * t2862 - t49244 - 6.0_f64 * t10740 * t14429 - 0.14035736694323150897e2_f64 * t10828 * t1581 * t10524 + 0.11579025239058625248e4_f64 * t10811 * t4437 * t10743 + 0.10526802520742363173e2_f64 * t10825 * t14463 - t49256 - t49259 - t49262 - 0.31168546390226634766e3_f64 * t49263 * t10724;
    t49266
}
