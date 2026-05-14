//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 794/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk794<F: Float>(t771: F, t219: F, t760: F, t777: F, t779: F, t712: F, t804: F, t244: F, t2977: F, t243: F, t2824: F, t40: F, t803: F, t901: F, t685: F, t790: F) -> (F, F, F, F, F, F, F) {
    let t11831 = t771 * t771;
    let t11834 = 6.0 * t760 * t11831 * t219;
    let t11837 = 0.48245938496077605201e2 * t777 * t11831 * t779;
    let t11841 = t712 * t804;
    let t11843 = t2977 * t244;
    let t11849 = t40 * t243 * t2824;
    let t11856 = t40 * t901 * t803;
    let t11869 = 1.0 / t685 / t790;
    (t11834, t11837, t11841, t11843, t11849, t11856, t11869)
}
