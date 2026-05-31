//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 451/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk451<F: Float>(t1831: F, t1833: F, t1845: F, t228: F, t659: F, t663: F) -> (F, F, F) {
    let t1847 = t1831 - F::cast_from(0.35616666666666666666e-1_f64) * t1833 + F::cast_from(0.53425e-1_f64) * t1845;
    let t1849 = F::cast_from(0.621814e-1_f64) * t1847 * t228;
    let t1850 = t659 * t663;
    (t1847, t1849, t1850)
}
