//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 505/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk505<F: Float>(t159: F, t1709: F, t1710: F, t1723: F, t1730: F, t1747: F, t1750: F, t1752: F, t1756: F, t1761: F, t1766: F, t1770: F, t1772: F, t1784: F, t1788: F, t1789: F, t41: F) -> (F,) {
    let t1791 = t1709 + 0.1301229756036208781e0 * t1710 + t1723 + t1730 + t1747 + t1750 - 0.10843581300301739842e-1 * t1752 - t1756 - t1761 + t1766 - t1770 - 0.40020429009866666666e-2 * t1772 + 0.285764e-1 * t159 * t1784 + t1788 - t41 * t1789;
    (t1791,)
}
