//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2922/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2922<F: Float>(t41329: F, t41361: F, t51978: F, t52082: F, t63276: F, t63278: F, t77499: F, t77503: F, t77505: F, t77507: F, t77509: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77539: F, t77543: F, t77547: F) -> F {
    let t77778 = F::new(10.0) / F::new(81.0) * t77499 - t77503 / F::new(3.0) + t77505 / F::new(9.0) - F::new(4.0) / F::new(9.0) * t77507 + F::new(2.0) / F::new(3.0) * t77509 - F::new(2.0) / F::new(3.0) * t63276 + F::new(2.0) / F::new(9.0) * t63278 + t41329 + F::new(4.0) * t77515 - F::new(10.0) / F::new(9.0) * t77518 - F::new(6.0) * t77521 - t52082 + F::new(28.0) / F::new(27.0) * t51978 + F::new(28.0) / F::new(81.0) * t41361 - F::new(2.0) / F::new(3.0) * t77527 - F::new(2.0) / F::new(3.0) * t77531 + F::new(8.0) * t77535 - F::new(6.0) * t77539 + F::new(2.0) * t77543 + F::new(2.0) * t77547;
    t77778
}
