//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 371/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk371<F: Float>(t435: F, t507: F, t561: F, t589: F, t195: F, t588: F, t169: F, t1036: F, t1037: F, t457: F, t505: F, t202: F) -> (F, F, F, F, F) {
    let t1823 = t435 * t507;
    let t1826 = t561 * t589;
    let t1829 = t588 * t195;
    let t1830 = t169 * t1829;
    let t1835 = t1036 * t1037 * t457 * t505;
    let t1838 = t202 * t202;
    let t1839 = 1.0 / t1838;
    (t1823, t1826, t1830, t1835, t1839)
}
