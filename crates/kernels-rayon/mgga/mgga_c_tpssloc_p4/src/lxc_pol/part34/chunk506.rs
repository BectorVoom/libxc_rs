//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 506/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk506(t1017: f64, t1742: f64, t1210: f64, t1207: f64, t372: f64, t479: f64, t471: f64, t1193: f64, t1706: f64, t135: f64, t1725: f64, t1174: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5017 = t1742 * t1017;
    let t5018 = t1210 * t5017;
    let t5019 = t1207 * t5018;
    let t5022 = t1742 * t372;
    let t5023 = t479 * t5022;
    let t5024 = t471 * t5023;
    let t5036 = t1706 * t1193;
    let t5040 = t135 * t1725;
    let t5041 = t1174 * t5040;
    (t5018, t5019, t5023, t5024, t5036, t5040, t5041)
}
