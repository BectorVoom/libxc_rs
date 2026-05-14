//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 976/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk976<F: Float>(t35100: F, t1131: F, t525: F, t1181: F, t2068: F, t604: F, t33706: F, t599: F, t1165: F, t21955: F, t30806: F, t23718: F, t7351: F, t7575: F, t4263: F, t8600: F) -> (F, F, F, F, F, F, F) {
    let t35101 = 0.12862205435420921092e-2 * t35100;
    let t35102 = t525 * t1131;
    let t35105 = t2068 * t1181 * t604 * t35102;
    let t35109 = t2068 * t1181 * t599 * t33706;
    let t35113 = t30806 * t1165 * t604 * t21955;
    let t35114 = 0.94344276868812456204e-2 * t35113;
    let t35117 = t7575 * t1181 * t7351 * t23718;
    let t35121 = t7575 * t1165 * t8600 * t4263;
    (t35101, t35102, t35105, t35109, t35114, t35117, t35121)
}
