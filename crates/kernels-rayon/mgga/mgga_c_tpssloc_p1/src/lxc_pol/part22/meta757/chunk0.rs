//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2542/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2542(t1102: f64, t21785: f64, t43889: f64, t18746: f64, t4756: f64, t14813: f64, t5999: f64, t71183: f64, t71187: f64, t71446: f64, t71449: f64, t71452: f64, t71454: f64, t71456: f64, t71458: f64) -> (f64, f64, f64, f64) {
    let t71461 = t43889 * t21785 * t1102;
    let t71463 = t18746 * t4756;
    let t71465 = t14813 * t5999;
    let t71467 = -0.60384999999999999999e0_f64 * t71183 - 0.60384999999999999999e0_f64 * t71187 + 0.82524375e-1_f64 * t71446 - 0.1294625e1_f64 * t71449 - 0.485484375e1_f64 * t71452 + 0.58258125e1_f64 * t71454 - 0.3883875e1_f64 * t71456 - 0.3883875e1_f64 * t71458 + 0.6189328125e-1_f64 * t71461 - 0.1237865625e0_f64 * t71463 + 0.247573125e0_f64 * t71465;
    (t71461, t71463, t71465, t71467)
}
