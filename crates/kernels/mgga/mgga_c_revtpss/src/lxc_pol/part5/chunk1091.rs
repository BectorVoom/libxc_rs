//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1091/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1091<F: Float>(t15045: F, t689: F, t4470: F, t786: F, t789: F, t4534: F, t779: F, t2435: F, t4322: F, t1596: F, t2873: F, t1614: F, t2942: F) -> (F, F, F, F, F, F) {
    let t15047 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t15045;
    let t15048 = t786 * t4470;
    let t15050 = F::cast_from(0.19514881078765566038e-1_f64) * t15048 * t789;
    let t15060 = t779 * t4534;
    let t15062 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t15060;
    let t15063 = t2435 * t4322;
    let t15101 = t1596 * t2873;
    let t15104 = t1614 * t2942;
    (t15047, t15050, t15062, t15063, t15101, t15104)
}
