//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1005/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1005<F: Float>(t23705: F, t2970: F, t15123: F, t15189: F, t23472: F, t23476: F, t23479: F, t23483: F, t23487: F, t23490: F, t23493: F, t23496: F, t23501: F, t23505: F, t23508: F, t23511: F) -> (F, F) {
    let t23723 = t23705 * t2970;
    let t23740 = -F::new(0.46308888888888888889e-1) * t23472 - F::new(0.104195e0) * t23476 - F::new(0.57386111111111111112e0) * t23479 + F::new(0.20659e1) * t23483 - F::new(0.309885e1) * t23487 - F::new(0.516475e0) * t23490 + F::new(0.20839e0) * t23493 - F::new(0.62517e0) * t23496 - F::new(0.34731666666666666667e0) * t15123 - F::new(0.103295e1) * t23501 + F::new(0.309885e1) * t23505 - F::new(0.104195e0) * t23508 + F::new(0.62517e0) * t23511 - F::new(0.68863333333333333332e0) * t15189;
    (t23723, t23740)
}
