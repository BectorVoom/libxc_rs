//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 772/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk772<F: Float>(t1187: F, t2877: F, t3668: F, t827: F, t3662: F, t3951: F, t79: F, t12831: F, t26: F, t1186: F, t12925: F, t3580: F, t821: F) -> (F, F, F, F, F, F) {
    let t12935 = t2877 * t1187;
    let t12937 = t827 * t3668;
    let t12939 = t827 * t3662;
    let t12941 = t79 * t3951;
    let t12942 = t12941 * t12831;
    let t12943 = t26 * t12942;
    let t12945 = t1186 * t12925;
    let t12946 = t26 * t12945;
    let t12948 = t821 * t3580;
    (t12935, t12937, t12939, t12943, t12946, t12948)
}
