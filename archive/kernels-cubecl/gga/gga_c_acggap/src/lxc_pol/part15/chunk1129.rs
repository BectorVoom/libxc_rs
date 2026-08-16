//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1129/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1129<F: Float>(t7839: F, t9633: F, t2068: F, t2263: F, t35137: F, t8480: F, t8521: F, t34345: F, t7585: F, t8525: F, t9637: F, t4680: F, t9636: F) -> (F, F, F, F, F, F) {
    let t39534 = t7839 * t9633;
    let t39537 = t2068 * t35137 * t2263;
    let t39540 = t2068 * t8480 * t8521;
    let t39545 = t7585 * t34345 * t8525;
    let t39547 = t7839 * t9637;
    let t39551 = t2068 * t4680 * t9636;
    (t39534, t39537, t39540, t39545, t39547, t39551)
}
