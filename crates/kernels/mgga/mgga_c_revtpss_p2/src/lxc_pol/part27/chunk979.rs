//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 979/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk979<F: Float>(t11876: F, t3117: F, t1016: F, t697: F, t1011: F, t1010: F, t2270: F, t3241: F, t3244: F, t1058: F, t3197: F, t11132: F) -> (F, F, F, F, F, F) {
    let t11877 = t3117 * t11876;
    let t11880 = t697 * t1016;
    let t11881 = t1011 * t11880;
    let t11883 = t2270 * t1010;
    let t11886 = t3241 * t3244;
    let t11888 = t3197 * t1058;
    let t11890 = F::cast_from(0.25925925925925925926e-1_f64) * t11132;
    (t11877, t11881, t11883, t11886, t11888, t11890)
}
