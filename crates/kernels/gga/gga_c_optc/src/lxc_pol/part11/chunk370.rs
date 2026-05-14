//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 370/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk370<F: Float>(t1810: F, t1828: F, t1772: F, t31: F, t4: F, t508: F, t514: F, t209: F, t535: F, t580: F, t579: F, t80: F) -> (F, F, F, F, F, F, F) {
    let t1829 = t1810 * t1828;
    let t1834 = 0.14764770444444444444e-2 * t4 * t1772 * t31;
    let t1835 = t508 * t514;
    let t1838 = 0.35616666666666666667e-1 * t209 * t1835 * t535;
    let t1842 = t508 * t580;
    let t1846 = t579 * t80;
    let t1847 = 1.0 / t1846;
    (t1829, t1834, t1835, t1838, t1842, t1846, t1847)
}
