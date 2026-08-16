//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 355/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk355(t1539: f64, t882: f64, t123: f64, t881: f64, t291: f64, t880: f64, t894: f64, t901: f64, t908: f64, t136: f64, t899: f64, t907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1540 = t882 * t1539;
    let t1541 = t123 * t1540;
    let t1543 = -t881 - 0.17808333333333333333e-1_f64 * t1541;
    let t1545 = 0.621814e-1_f64 * t1543 * t291;
    let t1547 = -t880 / 3.0_f64 - t1541 / 3.0_f64;
    let t1548 = t894 * t1547;
    let t1551 = t901 * t1547;
    let t1553 = t908 * t1539;
    let t1554 = t136 * t1553;
    let t1556 = 0.1898925e1_f64 * t1548 - t899 - 0.29896666666666666667e0_f64 * t1541 + 0.3071625e0_f64 * t1551 - t907 - 0.82156666666666666667e-1_f64 * t1554;
    (t1540, t1541, t1543, t1545, t1547, t1548, t1551, t1553, t1554, t1556)
}
