//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 843/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk843<F: Float>(t129: F, t5: F, t2248: F, t814: F, t2407: F, t813: F, t1077: F, t435: F, t965: F, t1159: F, t848: F, t1111: F, t301: F) -> (F, F, F, F, F, F, F, F) {
    let t10146 = t129 * t5;
    let t10761 = t814 * t2248;
    let t11179 = t814 * t2407;
    let t11882 = t813 * t813;
    let t11883 = F::cast_from(1.0_f64) / t11882;
    let t12473 = t435 * t1077;
    let t12610 = t965 * t435;
    let t12726 = t848 * t1159;
    let t12816 = t1111 * t301;
    (t10146, t10761, t11179, t11883, t12473, t12610, t12726, t12816)
}
