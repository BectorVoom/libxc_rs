//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 945/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk945<F: Float>(t15123: F, t15189: F, t23472: F, t23476: F, t23479: F, t23483: F, t23487: F, t23490: F, t23493: F, t23496: F, t23501: F, t23505: F, t23508: F, t23511: F) -> F {
    let t23680 = -F::new(0.36793333333333333333e-1) * t23472 - F::new(0.82785e-1) * t23476 - F::new(0.33547222222222222222e0) * t23479 + F::new(0.12077e1) * t23483 - F::new(0.181155e1) * t23487 - F::new(0.301925e0) * t23490 + F::new(0.16557e0) * t23493 - F::new(0.49671e0) * t23496 - F::new(0.27595e0) * t15123 - F::new(0.60384999999999999999e0) * t23501 + F::new(0.181155e1) * t23505 - F::new(0.82785e-1) * t23508 + F::new(0.49671e0) * t23511 - F::new(0.40256666666666666668e0) * t15189;
    t23680
}
