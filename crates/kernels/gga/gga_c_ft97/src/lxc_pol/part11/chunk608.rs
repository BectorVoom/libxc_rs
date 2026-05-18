//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 608/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk608<F: Float>(t24: F, t7241: F, t110: F, t7751: F, t486: F, t100: F, t1853: F, t492: F, t83: F, t1570: F, t487: F, t8211: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8411 = t24 * t7241;
    let t8413 = t8411 * t110 * t7751;
    let t8416 = t486 * t486;
    let t8417 = F::new(1.0) / t8416;
    let t8418 = t100 * t8417;
    let t8419 = t1853 * t492;
    let t8420 = t8418 * t8419;
    let t8421 = t83 * t8420;
    let t8424 = t487 * t1570;
    let t8425 = t8424 * t8211;
    (t8411, t8413, t8416, t8417, t8418, t8419, t8420, t8421, t8425)
}
