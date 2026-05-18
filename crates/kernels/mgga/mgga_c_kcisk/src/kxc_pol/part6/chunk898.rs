//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 898/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk898<F: Float>(t652: F, t743: F, t719: F, t717: F, t415: F, t2527: F, t8672: F, t1801: F, t11227: F, t1869: F, t6697: F, t5062: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t28957 = F::new(1.0) / t652 / t743;
    let t28958 = sigma2 * t28957;
    let t28959 = t28958 * t719;
    let t28960 = t717 * t28959;
    let t28961 = t415 * t28960;
    let t28963 = t8672 * t2527;
    let t28964 = t1801 * t28963;
    let t28965 = t11227 * t28964;
    let t28966 = t1869 * t28965;
    let t28968 = t6697 * t8672;
    let t28969 = t5062 * t28968;
    (t28957, t28958, t28961, t28963, t28966, t28969)
}
