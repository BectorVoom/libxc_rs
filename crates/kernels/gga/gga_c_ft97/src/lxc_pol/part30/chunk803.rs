//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 803/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk803<F: Float>(t665: F, t7514: F, t762: F, t9895: F, t2492: F, t2568: F, t754: F, t192: F, t33300: F, t2469: F, t2404: F, t2680: F, t683: F, t7640: F, t191: F, t33828: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42123 = t665 * t7514;
    let t42334 = t9895 * t762;
    let t42339 = t2492 * t2568;
    let t42376 = t9895 * t754;
    let t42500 = t192 * t33300;
    let t42575 = t2492 * t2469;
    let t43350 = t2404 * t2680;
    let t43381 = t683 * t7640;
    let t43524 = t191 * t33828;
    (t42123, t42334, t42339, t42376, t42500, t42575, t43350, t43381, t43524)
}
