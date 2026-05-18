//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1042/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1042<F: Float>(t59354: F, t59364: F, t86202: F, t86205: F, t86208: F, t86211: F, t86214: F, t86217: F, t86220: F, t86223: F, t86226: F, t86232: F, t86236: F, t86240: F) -> F {
    let t86453 = F::new(2.0) / F::new(9.0) * t86202 + t86205 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t86208 - F::new(4.0) / F::new(27.0) * t86211 + F::new(2.0) / F::new(9.0) * t86214 + F::new(20.0) / F::new(81.0) * t86217 - F::new(10.0) / F::new(27.0) * t86220 + F::new(4.0) / F::new(3.0) * t86223 + F::new(4.0) / F::new(3.0) * t86226 + t59354 - t59364 - F::new(6.0) * t86232 - F::new(4.0) / F::new(3.0) * t86236 - t86240 / F::new(9.0);
    t86453
}
