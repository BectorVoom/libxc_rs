//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3365/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3365<F: Float>(t41329: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F) -> F {
    let t63412 = F::new(4.0) / F::new(3.0) * t63274 - F::new(4.0) / F::new(9.0) * t63276 + F::new(4.0) / F::new(27.0) * t63278 - F::new(4.0) / F::new(9.0) * t63281 - F::new(2.0) / F::new(9.0) * t63285 - F::new(10.0) / F::new(27.0) * t63290 + F::new(4.0) / F::new(3.0) * t63293 + F::new(2.0) / F::new(3.0) * t63299 + F::new(40.0) / F::new(9.0) * t63304 - F::new(8.0) * t63308 + t41329 + F::new(2.0) / F::new(9.0) * t51967;
    t63412
}
