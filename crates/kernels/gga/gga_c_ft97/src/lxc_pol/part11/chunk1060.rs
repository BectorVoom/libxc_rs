//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1060/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1060<F: Float>(t2330: F, t2464: F, t263: F, t41403: F, t41405: F, t41411: F, t41414: F, t41417: F, t41419: F, t41421: F, t41821: F, t41875: F, t41929: F, t41983: F, t661: F, t771: F, t9511: F, t9512: F, t9780: F) -> F {
    let t41988 = -F::new(48.0) * t41403 + F::new(48.0) * t41405 + F::new(48.0) * t41411 - F::new(72.0) * t41414 + F::new(24.0) * t41417 - F::new(12.0) * t41419 - F::new(8.0) * t41421 - F::new(3.0) * t2330 * t9780 * t263 - F::new(3.0) * t9511 * t2464 * t263 - F::new(4.0) * t9512 * t771 - t661 * (t41821 + t41875 + t41929 + t41983) * t263;
    t41988
}
