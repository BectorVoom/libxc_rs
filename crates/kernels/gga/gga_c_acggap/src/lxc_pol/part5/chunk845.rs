//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 845/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk845<F: Float>(t11787: F, t2801: F, t779: F, t229: F, t2827: F, t771: F, t219: F, t760: F, t777: F, t712: F, t804: F, t244: F, t2977: F) -> (F, F, F, F, F, F) {
    let t11828 = F::cast_from(0.57895126195293126241e3_f64) * t2801 * t11787 * t779;
    let t11829 = t229 * t2827;
    let t11831 = t771 * t771;
    let t11834 = F::new(6.0) * t760 * t11831 * t219;
    let t11837 = F::cast_from(0.48245938496077605201e2_f64) * t777 * t11831 * t779;
    let t11841 = t712 * t804;
    let t11843 = t2977 * t244;
    (t11828, t11829, t11834, t11837, t11841, t11843)
}
