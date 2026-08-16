//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1268/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1268(t120521: f64, t114116: f64, t114121: f64, t114104: f64, t114119: f64, t114130: f64, t120425: f64, t120491: f64, t120496: f64, t120502: f64, t120505: f64, t120506: f64, t120507: f64, t120513: f64, t120515: f64, t120516: f64, t1332: f64, t1336: f64, t1352: f64, t1825: f64, t32753: f64, t32755: f64, t3777: f64, t5250: f64, t5334: f64, t5344: f64, t544: f64, t553: f64) -> f64 {
    let t120522 = 0.82246703342411321825e-2_f64 * t120521;
    let t120525 = 0.38381794893125283518e-1_f64 * t114116;
    let t120526 = 0.82246703342411321825e-2_f64 * t114121;
    let t120528 = -t114130 * t1336 * t1825 + t120425 * t544 * t553 - t120516 * t1352 * t5344 + 2.0_f64 * t120516 * t5250 * t5334 + t1332 * t32755 - t32753 * t3777 + t114104 + t114119 + t120491 - t120496 - t120502 + t120505 - t120506 + t120507 + t120513 - t120515 - t120522 - t120525 + t120526;
    t120528
}
