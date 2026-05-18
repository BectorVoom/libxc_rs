//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 543/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk543<F: Float>(t1613: F, t5551: F, t5555: F, t1609: F, t58: F, t1293: F, t1710: F, t6: F, t8051: F, t8: F, t3076: F, t12: F, t391: F, sigma0: F) -> (F, F, F, F, F) {
    let t22737 = t1613 * t5551;
    let t22738 = t22737 * t5555;
    let t22742 = t1609 * sigma0;
    let t22743 = t22742 * t58;
    let t22755 = t1710 * t1293;
    let t22759 = t8051 * t6;
    let t22760 = t22759 * t8;
    let t22761 = t3076 * t22760;
    let t22766 = t12 * t391;
    (t22738, t22743, t22755, t22761, t22766)
}
