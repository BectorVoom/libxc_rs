//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 320/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk320<F: Float>(t1824: F, t1887: F, t706: F, t1421: F, t1689: F, t1875: F, t1879: F, t1884: F, t456: F, t604: F) -> (F, F, F) {
    let t1888 = t1887 * t1824;
    let t1889 = t706 * t1888;
    let t1894 = t1875 + 0.65704296666666666667e-3 * t1421 * t1879 + 0.1478346675e-2 * t456 * t1884 - 0.98556445e-3 * t456 * t1889 - 4.0 * t604 * t1689;
    (t1888, t1889, t1894)
}
