//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 841/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk841<F: Float>(t1067: F, t31993: F, t25669: F, t8513: F, t1078: F, t3143: F, t1039: F) -> (F, F, F, F) {
    let t31994 = t31993 * t1067;
    let t31997 = t8513 * t25669;
    let t31998 = t1078 * t3143;
    let t31999 = t31998 * t1039;
    (t31994, t31997, t31998, t31999)
}
