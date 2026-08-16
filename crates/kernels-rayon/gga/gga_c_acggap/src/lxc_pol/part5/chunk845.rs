//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 845/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk845(t11787: f64, t2801: f64, t779: f64, t229: f64, t2827: f64, t771: f64, t219: f64, t760: f64, t777: f64, t712: f64, t804: f64, t244: f64, t2977: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11828 = 0.57895126195293126241e3_f64 * t2801 * t11787 * t779;
    let t11829 = t229 * t2827;
    let t11831 = t771 * t771;
    let t11834 = 6.0_f64 * t760 * t11831 * t219;
    let t11837 = 0.48245938496077605201e2_f64 * t777 * t11831 * t779;
    let t11841 = t712 * t804;
    let t11843 = t2977 * t244;
    (t11828, t11829, t11834, t11837, t11841, t11843)
}
