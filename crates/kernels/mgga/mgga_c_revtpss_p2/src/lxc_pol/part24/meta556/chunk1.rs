//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1662/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1662<F: Float>(t51978: F, t77505: F, t77507: F, t77509: F, t88104: F, t88108: F, t88114: F, t88118: F, t88122: F, t88126: F, t88130: F, t88134: F) -> F {
    let t88201 = F::new(4.0) / F::new(9.0) * t77505 - F::new(80.0) / F::new(81.0) * t88104 - t88108 / F::new(3.0) - F::new(16.0) / F::new(9.0) * t77507 + F::new(8.0) / F::new(3.0) * t77509 + F::new(40.0) / F::new(9.0) * t88114 - F::new(20.0) / F::new(9.0) * t88118 - F::new(8.0) * t88122 + F::new(8.0) * t88126 - F::new(2.0) / F::new(3.0) * t88130 - F::new(8.0) / F::new(9.0) * t88134 + F::new(112.0) / F::new(81.0) * t51978;
    t88201
}
