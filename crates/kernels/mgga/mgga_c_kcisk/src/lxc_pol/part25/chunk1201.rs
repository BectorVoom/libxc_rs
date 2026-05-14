//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1201/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1201<F: Float>(t34656: F, t567: F, t564: F, t2356: F, t9777: F, t10043: F, t1629: F, t9636: F, t9904: F, t9642: F, t566: F, t7694: F, t2776: F, t2360: F, t9776: F, t2819: F, t6651: F) -> (F, F, F, F, F, F, F, F) {
    let t34657 = t567 * t34656;
    let t34658 = t564 * t34657;
    let t34660 = t2356 * t9777;
    let t34662 = t1629 * t10043;
    let t34663 = t564 * t34662;
    let t34665 = t9904 * t9636;
    let t34668 = t2356 * t9642;
    let t34670 = t566 * t7694;
    let t34671 = t2776 * t34670;
    let t34673 = t2360 * t9776;
    let t34674 = t564 * t34673;
    let t34676 = t6651 * t2819;
    (t34658, t34660, t34663, t34665, t34668, t34671, t34674, t34676)
}
