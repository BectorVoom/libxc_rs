//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3383/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3383<F: Float>(t63262: F, t63295: F, t63334: F, t63380: F, t63473: F, t63509: F, t63540: F, t63573: F, t915: F, t935: F, t41578: F, t6145: F) -> (F, F) {
    let t63579 = F::new(1.0) * t915 * (t63262 + t63295 + t63334 + t63380 + t63473 + t63509 + t63540 + t63573) * t935;
    let t63581 = F::cast_from(0.16081979498692535067e2_f64) * t41578 * t6145;
    (t63579, t63581)
}
