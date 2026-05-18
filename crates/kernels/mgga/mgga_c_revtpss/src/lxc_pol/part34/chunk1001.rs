//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1001/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1001<F: Float>(t11534: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F, t291: F) -> F {
    let t23663 = -t11534 - F::new(0.23744444444444444444e-1) * t15189 + F::new(0.11872222222222222222e-1) * t18919 - F::new(0.35616666666666666666e-1) * t18924 + F::new(0.17808333333333333333e-1) * t18934 - F::new(0.19787037037037037037e-1) * t23479 + F::new(0.71233333333333333332e-1) * t23483 - F::new(0.35616666666666666666e-1) * t23501 - F::new(0.10685e0) * t23487 + F::new(0.10685e0) * t23505 - F::new(0.17808333333333333333e-1) * t23490;
    let t23665 = F::new(0.621814e-1) * t23663 * t291;
    t23665
}
