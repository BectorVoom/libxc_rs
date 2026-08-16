//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1511/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1511(t1049: f64, t3040: f64, t3188: f64, t10857: f64, t381: f64, t1060: f64, t1022: f64, t3166: f64, t10947: f64, t3185: f64) -> (f64, f64, f64, f64, f64) {
    let t11023 = t1049 * t3040;
    let t11024 = t11023 * t3188;
    let t11027 = t381 * t10857;
    let t11028 = t11027 * t1060;
    let t11030 = t3166 * t1022;
    let t11031 = t11030 * t1060;
    let t11034 = t10947 * t3185;
    (t11023, t11024, t11028, t11031, t11034)
}
