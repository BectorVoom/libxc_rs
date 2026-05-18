//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 586/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk586<F: Float>(t231: F, t3817: F, t3750: F, t679: F, t200: F, t1095: F, t3773: F, t6027: F, t1613: F, t6789: F, t6793: F, t17836: F, t6033: F) -> (F, F, F, F, F) {
    let t27588 = t231 * t3817;
    let t27595 = t679 * t3750;
    let t27596 = t27595 * t200;
    let t27601 = t3773 * t6027 * t1095;
    let t27604 = t1613 * t6789;
    let t27605 = t27604 * t6793;
    let t27609 = t17836 * t6033;
    (t27588, t27596, t27601, t27605, t27609)
}
