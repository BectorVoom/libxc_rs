//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1023/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1023<F: Float>(t11534: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F, t291: F) -> F {
    let t23663 = -t11534 - F::cast_from(0.23744444444444444444e-1_f64) * t15189 + F::cast_from(0.11872222222222222222e-1_f64) * t18919 - F::cast_from(0.35616666666666666666e-1_f64) * t18924 + F::cast_from(0.17808333333333333333e-1_f64) * t18934 - F::cast_from(0.19787037037037037037e-1_f64) * t23479 + F::cast_from(0.71233333333333333332e-1_f64) * t23483 - F::cast_from(0.35616666666666666666e-1_f64) * t23501 - F::cast_from(0.10685e0_f64) * t23487 + F::cast_from(0.10685e0_f64) * t23505 - F::cast_from(0.17808333333333333333e-1_f64) * t23490;
    let t23665 = F::cast_from(0.621814e-1_f64) * t23663 * t291;
    t23665
}
