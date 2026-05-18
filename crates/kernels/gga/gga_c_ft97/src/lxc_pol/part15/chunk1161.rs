//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1161/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1161<F: Float>(t41951: F, t80770: F, t80772: F, t80819: F, t80821: F, t88221: F, t88225: F, t88229: F, t89027: F, t89030: F, t89034: F, t89038: F, t89042: F, t89047: F) -> F {
    let t89727 = F::new(4.0) / F::new(27.0) * t80770 - F::new(4.0) / F::new(27.0) * t80772 + t41951 + F::new(2.0) / F::new(9.0) * t88221 - F::new(2.0) / F::new(3.0) * t88225 - F::new(4.0) / F::new(9.0) * t88229 - F::new(2.0) / F::new(9.0) * t80819 - F::new(2.0) / F::new(9.0) * t80821 - F::new(10.0) / F::new(27.0) * t89027 - F::new(2.0) * t89030 - F::new(4.0) / F::new(3.0) * t89034 - t89038 / F::new(9.0) + t89042 / F::new(3.0) - F::new(40.0) / F::new(243.0) * t89047;
    t89727
}
