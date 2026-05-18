//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1269/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1269<F: Float>(t1416: F, t3116: F, t9180: F, t13853: F, t169: F, t21204: F, t4043: F, t519: F, t11430: F, t3060: F, t8716: F, t11350: F, t9241: F) -> (F, F, F, F) {
    let t35019 = t9180 * t1416 * t3116;
    let t35024 = t169 * t21204 * t4043 * t519 * t13853;
    let t35027 = t3060 * t11430 * t8716;
    let t35031 = t11350 * t9241;
    (t35019, t35024, t35027, t35031)
}
