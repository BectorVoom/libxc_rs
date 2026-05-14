//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 777/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk777<F: Float>(t10004: F, t10038: F, t105: F, t469: F, t182: F, t310: F, t129: F, t5: F, t2248: F, t814: F, t2407: F, t813: F, t1159: F, t848: F, t862: F, t1016: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10039 = t10004 + t10038;
    let t10040 = t105 * t10039;
    let t10041 = t10040 * t469;
    let t10098 = t310 * t182;
    let t10146 = t129 * t5;
    let t10761 = t814 * t2248;
    let t11179 = t814 * t2407;
    let t11882 = t813 * t813;
    let t11883 = 1.0 / t11882;
    let t12726 = t848 * t1159;
    let t12935 = t862 * t182;
    let t13067 = t360 * t1016;
    (t10039, t10040, t10041, t10098, t10146, t10761, t11179, t11883, t12726, t12935, t13067)
}
