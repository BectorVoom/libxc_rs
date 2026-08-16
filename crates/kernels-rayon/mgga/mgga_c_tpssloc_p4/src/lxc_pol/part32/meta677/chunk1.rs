//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2115/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2115(t24574: f64, t27481: f64, t7365: f64, t94490: f64, t1715: f64, t974: f64, t24847: f64, t24771: f64, t7999: f64, t15418: f64, t2127: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95751 = 0.54831135561607547884e-2_f64 * t24574 * t27481;
    let t95758 = t94490 * t7365;
    let t95760 = t974 * t1715;
    let t95761 = t24847 * t95760;
    let t95768 = t7999 * t24771;
    let t95772 = t2127 * t221 * t15418;
    (t95751, t95758, t95760, t95761, t95768, t95772)
}
