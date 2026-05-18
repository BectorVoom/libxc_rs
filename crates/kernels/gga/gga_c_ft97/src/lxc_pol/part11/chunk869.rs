//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 869/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk869<F: Float>(t37357: F, t37789: F, t419: F, t420: F, t1725: F, t8098: F, t1743: F, t626: F, t8115: F, t8122: F, t1737: F, t37362: F) -> (F, F, F, F, F, F) {
    let t37792 = t419 * t420 * t37789 * t37357;
    let t37795 = t1725 * t8098;
    let t37798 = t419 * t626 * t1743;
    let t37800 = t1725 * t8115;
    let t37802 = t1725 * t8122;
    let t37806 = t419 * t420 * t1737 * t37362;
    (t37792, t37795, t37798, t37800, t37802, t37806)
}
