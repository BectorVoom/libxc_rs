//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 940/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk940<F: Float>(t11534: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F, t291: F, t15123: F, t23472: F, t23476: F, t23493: F, t23496: F, t23508: F, t23511: F) -> (F, F) {
    let t23663 = -t11534 - 0.23744444444444444444e-1 * t15189 + 0.11872222222222222222e-1 * t18919 - 0.35616666666666666666e-1 * t18924 + 0.17808333333333333333e-1 * t18934 - 0.19787037037037037037e-1 * t23479 + 0.71233333333333333332e-1 * t23483 - 0.35616666666666666666e-1 * t23501 - 0.10685e0 * t23487 + 0.10685e0 * t23505 - 0.17808333333333333333e-1 * t23490;
    let t23665 = 0.621814e-1 * t23663 * t291;
    let t23680 = -0.36793333333333333333e-1 * t23472 - 0.82785e-1 * t23476 - 0.33547222222222222222e0 * t23479 + 0.12077e1 * t23483 - 0.181155e1 * t23487 - 0.301925e0 * t23490 + 0.16557e0 * t23493 - 0.49671e0 * t23496 - 0.27595e0 * t15123 - 0.60384999999999999999e0 * t23501 + 0.181155e1 * t23505 - 0.82785e-1 * t23508 + 0.49671e0 * t23511 - 0.40256666666666666668e0 * t15189;
    (t23665, t23680)
}
