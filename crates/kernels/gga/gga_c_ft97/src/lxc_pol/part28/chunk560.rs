//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 560/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk560<F: Float>(t1852: F, t25590: F, t10969: F, t5731: F, t1332: F, t3255: F, t492: F, t6557: F, t379: F, t6421: F, t22907: F, t22908: F, t3204: F, t1308: F, t378: F, t108: F, t1570: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t25591 = t1852 * t25590;
    let t25593 = t10969 * t5731;
    let t25595 = t1332 * t3255;
    let t25596 = t1852 * t25595;
    let t25598 = t6557 * t492;
    let t25599 = t1852 * t25598;
    let t25601 = t6421 * t379;
    let t25602 = t22907 * t25601;
    let t25605 = t22908 * t3204;
    let t25606 = t22907 * t25605;
    let t25609 = t378 * t1308;
    let t25610 = t108 * t1570;
    (t25591, t25593, t25595, t25596, t25598, t25599, t25601, t25602, t25605, t25606, t25609, t25610)
}
