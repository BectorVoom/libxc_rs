//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1117/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1117(t14630: f64, t1629: f64, t14526: f64, t383: f64, t1022: f64, t4657: f64, t1060: f64, t14626: f64, t3188: f64, t1057: f64, t14205: f64, t11054: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14631 = t1629 * t14630;
    let t14640 = t383 * t14526;
    let t14644 = t4657 * t1022;
    let t14645 = t14644 * t1060;
    let t14648 = t14626 * t3188;
    let t14651 = t14205 * t1057;
    let t14654 = t1629 * t11054;
    (t14631, t14640, t14645, t14648, t14651, t14654)
}
