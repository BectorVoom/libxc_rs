//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1069/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1069<F: Float>(t31543: F, t31556: F, t1537: F, t14800: F, t31525: F, t1212: F, t30551: F, t14757: F, t14798: F, t1529: F, t21764: F, t21869: F, t21872: F, t2297: F, t27627: F, t30564: F, t30567: F, t31509: F, t31512: F, t31515: F, t31518: F, t31526: F, t4436: F, t4461: F, t4471: F, t4478: F, t6518: F, t8350: F, t8366: F, t8369: F, t8375: F) -> F {
    let t31557 = t31543 + t31556;
    let t31558 = t31557 * t1537;
    let t31561 = t31525 * t14800;
    let t31568 = t31525 * t1537;
    let t31573 = t30551 * t1212;
    let t31576 = F::cast_from(0.51947267698127589897e2_f64) * t4478 * t31509 - F::new(6.0) * t4436 * t31512 + F::cast_from(0.96494049533612093922e2_f64) * t4461 * t31515 - F::cast_from(0.35089340384731224426e1_f64) * t4471 * t31518 + F::new(3.0) * t6518 * t8366 + F::cast_from(0.96494049533612093922e2_f64) * t21764 * t8369 - F::cast_from(0.19298809906722418785e3_f64) * t14757 * t31526 + F::new(1.0) * t1529 * t31558 + F::cast_from(0.20691336878655965246e4_f64) * t14798 * t31561 + F::cast_from(0.17544670192365612213e1_f64) * t27627 * t2297 - F::new(6.0) * t21869 * t8350 + F::new(6.0) * t4461 * t31568 - F::cast_from(0.35089340384731224426e1_f64) * t21872 * t8375 + F::cast_from(0.35089340384731224426e1_f64) * t4478 * t31573 + t30564 - t30567;
    t31576
}
