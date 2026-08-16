//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1136/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1136(t18047: f64, t383: f64, t4684: f64, t5932: f64, t3188: f64, t4649: f64, t1629: f64, t4673: f64, t1625: f64, t1060: f64, t1022: f64, t5914: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18129 = t383 * t18047;
    let t18131 = t5932 * t4684;
    let t18138 = t3188 * t4649;
    let t18139 = t1629 * t18138;
    let t18142 = t5932 * t4673;
    let t18150 = t1625 * t4649;
    let t18151 = t18150 * t1060;
    let t18154 = t5914 * t1022;
    (t18129, t18131, t18139, t18142, t18151, t18154)
}
