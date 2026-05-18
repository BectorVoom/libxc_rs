//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 439/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk439<F: Float>(t164: F, t1753: F, t51: F, t592: F, t1720: F, t1540: F, t66: F, t168: F, t167: F, t180: F, t1726: F) -> (F, F, F, F, F, F) {
    let t1756 = t592 * t51 * t1753 * t164;
    let t1760 = t592 * t1720 * t164;
    let t1764 = F::new(1.0) / t66 / t1540;
    let t1765 = t168 * t1764;
    let t1768 = F::new(0.56688979511669985553e-2) * t167 * t1765 * t180;
    let t1769 = t167 * t1726;
    (t1756, t1760, t1764, t1765, t1768, t1769)
}
