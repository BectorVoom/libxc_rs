//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 919/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk919<F: Float>(t6412: F, t6764: F, t1312: F, t1518: F, t4248: F, t5877: F, t5883: F, t5920: F, t93: F, t5545: F, t5547: F, t5570: F) -> (F, F, F, F, F) {
    let t6765 = t6412 + t6764;
    let t6773 = F::new(2.0) * t1312 * t5920 + F::new(4.0) * t1518 * t4248 + F::new(2.0) * t5883 * t93 + t5877;
    let t6777 = F::new(8.0) * t5545;
    let t6778 = F::new(8.0) * t5547;
    let t6779 = F::new(2.0) * t5570;
    (t6765, t6773, t6777, t6778, t6779)
}
