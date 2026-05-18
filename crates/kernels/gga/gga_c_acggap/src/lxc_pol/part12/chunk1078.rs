//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1078/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1078<F: Float>(t1181: F, t20311: F, t7351: F, t7426: F, t1165: F, t21118: F, t8600: F, t5209: F, t7822: F, t7637: F, t8555: F, t12610: F, t1426: F, t2297: F, t598: F) -> (F, F, F, F, F) {
    let t35194 = t7426 * t1181 * t7351 * t20311;
    let t35198 = t7426 * t1165 * t8600 * t21118;
    let t35200 = t7822 * t5209;
    let t35204 = t7637 * t8555;
    let t35208 = t598 * t1426 * t12610 * t2297;
    (t35194, t35198, t35200, t35204, t35208)
}
