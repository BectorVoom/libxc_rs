//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1051/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1051<F: Float>(t2764: F, t2770: F, t895: F, t9016: F, t224: F, t227: F, t9015: F, t2718: F, t2724: F, t873: F, t8913: F, t2727: F, t206: F, t220: F, t8942: F, t870: F, t8943: F) -> (F, F, F, F, F, F, F, F) {
    let t36436 = t2764 * t2770;
    let t36439 = t895 * t9016;
    let t36513 = t224 / t9015 / t227;
    let t36533 = t2718 * t2724;
    let t36543 = t8913 * t873;
    let t36901 = t2727 * t2727;
    let t36902 = 1.0 / t36901;
    let t36908 = t206 / t8942 / t220;
    let t36936 = t870 * t8943;
    (t36436, t36439, t36513, t36533, t36543, t36902, t36908, t36936)
}
