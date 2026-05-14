//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1134/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1134<F: Float>(t1032: F, t6041: F, t867: F, t786: F, t7060: F, t105936: F, t92843: F, t29658: F, t686: F, t72: F, t7058: F, t7064: F, t27186: F, t99404: F, t98849: F, t29643: F) -> (F, F, F, F, F, F, F, F, F) {
    let t105944 = t6041 * t1032;
    let t105945 = t105944 * t867;
    let t105946 = t786 * t105945;
    let t105947 = t105946 * t7060;
    let t105949 = t92843 * t105936;
    let t105953 = t29658 * t72 * t686;
    let t105954 = t7058 * t105953;
    let t105956 = t7064 * t105953;
    let t105960 = t99404 * t27186;
    let t105962 = t98849 * t27186;
    let t105973 = t29643 * t72 * t686;
    (t105944, t105945, t105947, t105949, t105954, t105956, t105960, t105962, t105973)
}
