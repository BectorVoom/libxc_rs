//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 893/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk893(t2888: f64, t5758: f64, t10629: f64, t5774: f64, t225: f64, t5849: f64, t5851: f64, t1040: f64, t5904: f64, t248: f64, t3101: f64, t5867: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17547 = t5758 * t2888;
    let t17564 = t10629 * t5774;
    let t17575 = t5849 * t225;
    let t17588 = t5851 * t225;
    let t17607 = t5904 * t1040;
    let t17611 = t248 * t3101 * t5867;
    (t17547, t17564, t17575, t17588, t17607, t17611)
}
