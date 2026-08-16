//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1324/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1324<F: Float>(t224: F, t37496: F, t37642: F, t38080: F, t38515: F, t2036: F, t37339: F, t37342: F, t37344: F, t37346: F, t37349: F, t37352: F, t37354: F, t37356: F, t37478: F, t37644: F, t37649: F, t38060: F, t3899: F) -> F {
    let t38518 = t224 * (t37496 + t37642 + t38080 + t38515);
    let t38519 = t2036 * t3899 - t37339 - t37342 - t37344 + t37346 + t37349 - t37352 + t37354 + t37356 + t37478 - t37644 + t37649 + t38060 + t38518;
    t38519
}
