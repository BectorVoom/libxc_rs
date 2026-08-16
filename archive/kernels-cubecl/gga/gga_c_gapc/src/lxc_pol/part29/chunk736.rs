//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 736/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk736<F: Float>(t1912: F, t3045: F, t5285: F, t511: F, t670: F, t22: F, t1900: F, t1743: F, t5703: F, t3103: F, t577: F, t3109: F) -> (F, F, F, F, F, F) {
    let t8650 = t5285 * t3045 * t1912;
    let t8652 = t670 * t511;
    let t8654 = F::cast_from(1.0_f64) / t22 / t8652;
    let t8655 = t1900 * t8654;
    let t8657 = t1743 * t8655 * t5703;
    let t8659 = t577 * t3103;
    let t8660 = t8659 * t3109;
    (t8650, t8652, t8654, t8655, t8657, t8660)
}
