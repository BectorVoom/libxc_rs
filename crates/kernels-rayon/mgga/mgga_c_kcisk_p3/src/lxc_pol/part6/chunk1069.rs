//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1069/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1069(t31543: f64, t31556: f64, t1537: f64, t14800: f64, t31525: f64, t1212: f64, t30551: f64, t14757: f64, t14798: f64, t1529: f64, t21764: f64, t21869: f64, t21872: f64, t2297: f64, t27627: f64, t30564: f64, t30567: f64, t31509: f64, t31512: f64, t31515: f64, t31518: f64, t31526: f64, t4436: f64, t4461: f64, t4471: f64, t4478: f64, t6518: f64, t8350: f64, t8366: f64, t8369: f64, t8375: f64) -> f64 {
    let t31557 = t31543 + t31556;
    let t31558 = t31557 * t1537;
    let t31561 = t31525 * t14800;
    let t31568 = t31525 * t1537;
    let t31573 = t30551 * t1212;
    let t31576 = 0.51947267698127589897e2_f64 * t4478 * t31509 - 6.0_f64 * t4436 * t31512 + 0.96494049533612093922e2_f64 * t4461 * t31515 - 0.35089340384731224426e1_f64 * t4471 * t31518 + 3.0_f64 * t6518 * t8366 + 0.96494049533612093922e2_f64 * t21764 * t8369 - 0.19298809906722418785e3_f64 * t14757 * t31526 + 1.0_f64 * t1529 * t31558 + 0.20691336878655965246e4_f64 * t14798 * t31561 + 0.17544670192365612213e1_f64 * t27627 * t2297 - 6.0_f64 * t21869 * t8350 + 6.0_f64 * t4461 * t31568 - 0.35089340384731224426e1_f64 * t21872 * t8375 + 0.35089340384731224426e1_f64 * t4478 * t31573 + t30564 - t30567;
    t31576
}
