//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 982/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk982<F: Float>(t1937: F, t33602: F, t6985: F, t7735: F, t13272: F, t8435: F, t1497: F, t8441: F, t8621: F, t1469: F, t32143: F, t7714: F) -> (F, F, F, F, F, F, F) {
    let t33603 = t33602 * t1937;
    let t33605 = t6985 * t7735;
    let t33609 = t13272 * t8435;
    let t33612 = t8441 * t1497;
    let t33613 = t8621 * t33612;
    let t33617 = t8621 * t32143 * t1469;
    let t33620 = t8441 * t7714;
    (t33603, t33605, t33609, t33612, t33613, t33617, t33620)
}
