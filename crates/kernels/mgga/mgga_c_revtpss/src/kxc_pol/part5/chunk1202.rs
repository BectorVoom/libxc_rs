//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1202/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1202<F: Float>(t15416: F, t1610: F, t4590: F, t4632: F, t11134: F, t11534: F, t15127: F, t15189: F, t15503: F, t15504: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> (F, F, F) {
    let t19060 = F::new(2.0) * t15416 * t1610;
    let t19062 = F::new(2.0) * t4590 * t4632;
    let t19077 = -t11534 - F::new(0.79148148148148148147e-2) * t11134 - F::new(0.15829629629629629629e-1) * t15189 + F::new(0.79148148148148148147e-2) * t15127 - t15503 + t15504 + F::new(0.39574074074074074073e-2) * t18919 - F::new(0.19787037037037037037e-1) * t18906 + F::new(0.71233333333333333332e-1) * t18911 - F::new(0.23744444444444444444e-1) * t18915 - F::new(0.11872222222222222222e-1) * t18924 - F::new(0.10685e0) * t18928 + F::new(0.71233333333333333332e-1) * t18932 + F::new(0.5936111111111111111e-2) * t18934 - F::new(0.11872222222222222222e-1) * t18939 + F::new(0.35616666666666666666e-1) * t18944 - F::new(0.17808333333333333333e-1) * t18948;
    (t19060, t19062, t19077)
}
