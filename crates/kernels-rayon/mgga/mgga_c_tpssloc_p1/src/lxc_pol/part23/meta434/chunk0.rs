//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1273/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1273(t15503: f64, t18356: f64, t18975: f64, t5024: f64, t1174: f64, t21749: f64, t3431: f64, t135: f64, t22011: f64, t18375: f64, t5019: f64, t18329: f64, t4889: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t72632 = t15503 * t18356;
    let t72634 = t5024 * t18975;
    let t72648 = t1174 * t3431 * t21749;
    let t72669 = t1174 * t135 * t22011;
    let t72673 = t5019 * t18375;
    let t72703 = t4889 * t18329;
    (t72632, t72634, t72648, t72669, t72673, t72703)
}
