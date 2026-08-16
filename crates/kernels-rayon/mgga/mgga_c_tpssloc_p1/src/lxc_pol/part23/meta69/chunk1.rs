//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 417/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk417(t1799: f64, t210: f64, t214: f64, t1313: f64, t1315: f64, t1322: f64) -> (f64, f64) {
    let t1804 = t210 * t214 * t1799;
    let t1807 = -t1313 - 0.16666666666666666666e-2_f64 * t1315 * t1804 - t1322;
    (t1804, t1807)
}
