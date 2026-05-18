//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 858/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk858<F: Float>(t1992: F, t7585: F, t7842: F, t930: F, t2067: F, t4180: F, t7836: F, t3427: F, t7647: F, t7419: F, t7839: F, t1530: F, t7584: F) -> (F, F, F, F, F, F) {
    let t30118 = t7585 * t7842 * t1992 * t930;
    let t30120 = t4180 * t2067;
    let t30121 = t30120 * t7836;
    let t30123 = t7647 * t3427;
    let t30125 = t7839 * t7419;
    let t30127 = t1530 * t7584;
    (t30118, t30120, t30121, t30123, t30125, t30127)
}
