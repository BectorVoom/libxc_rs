//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1402/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1402(t11147: f64, t75836: f64, t136: f64, t3297: f64, t11153: f64, t1113: f64, t1089: f64, t75912: f64, t1088: f64, t123: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77973 = t11147 * t75836;
    let t77975 = t136 * t3297 * t77973;
    let t77977 = t11153 * t75836;
    let t77979 = t136 * t1113 * t77977;
    let t77981 = t1089 * t75912;
    let t77983 = t136 * t1113 * t77981;
    let t77989 = t123 * t1088 * t77977;
    (t77973, t77975, t77977, t77979, t77981, t77983, t77989)
}
