//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1394/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1394(t1615: f64, t3188: f64, t5872: f64, t5914: f64, t381: f64, t76740: f64, t11046: f64, t11048: f64, t11065: f64, t1610: f64, t1632: f64, t21481: f64, t21615: f64, t21622: f64, t21627: f64, t21634: f64, t21647: f64, t3131: f64, t3186: f64, t3200: f64, t3201: f64, t43553: f64, t43554: f64, t4669: f64, t47841: f64, t5936: f64, t77782: f64, t77794: f64) -> (f64, f64, f64) {
    let t77806 = t3188 * t1615;
    let t77819 = t5914 * t5872;
    let t77826 = t381 * t76740;
    let t77835 = -36.0_f64 * t11065 * t3131 * t5936 * t77794 + 4.0_f64 * t11046 * t11048 * t77782 - 4.0_f64 * t21622 * t21634 * t3200 + 8.0_f64 * t21634 * t3186 * t77806 + 12.0_f64 * t3186 * t3188 * t77819 - 6.0_f64 * t3200 * t3201 * t77819 - 36.0_f64 * t43553 * t43554 * t77826 + 4.0_f64 * t1610 * t21615 + 4.0_f64 * t1632 * t21481 + 12.0_f64 * t21627 * t4669 + 24.0_f64 * t21647 * t47841;
    (t77806, t77826, t77835)
}
