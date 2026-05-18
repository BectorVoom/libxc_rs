//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 940/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk940<F: Float>(t9468: F, t9474: F, t9478: F, t9481: F, t9483: F, t9486: F, t9488: F, t9491: F, t9494: F, t9499: F, t9502: F, t9505: F, t9509: F) -> F {
    let t10842 = -F::new(0.2471588561924985691e-3) * t9468 - F::new(0.82386285397499523032e-5) * t9474 + F::new(0.6746961805555555556e-5) * t9478 - F::new(0.4637672555408563478e-4) * t9481 - F::new(0.21642471925239962898e-3) * t9483 - F::new(0.11254699860307667372e-6) * t9486 + F::new(0.55603792169291016668e-2) * t9488 - F::new(0.20240885416666666668e-4) * t9491 - F::new(0.20240885416666666668e-4) * t9494 - F::new(0.22202903123154399017e-4) * t9499 + F::new(0.11272120794395814009e-6) * t9502 - F::new(0.20041830772435757309e-6) * t9505 + F::new(0.55603792169291016668e-2) * t9509;
    t10842
}
