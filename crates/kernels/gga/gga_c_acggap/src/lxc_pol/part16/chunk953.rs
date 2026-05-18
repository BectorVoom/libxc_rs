//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 953/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk953<F: Float>(t2299: F, t7610: F, t2310: F, t2294: F, t7780: F, t1524: F, t355: F, t1083: F, t1980: F, t7458: F, t535: F, t7457: F, t7459: F) -> (F, F, F, F, F, F, F) {
    let t33869 = t7610 * t2299;
    let t33874 = t7610 * t2310;
    let t33876 = t7780 * t2294;
    let t33883 = t355 * t1524;
    let t33884 = t1083 * t33883;
    let t33886 = t1980 * t7458 * t33884;
    let t33887 = F::new(0.28582678745379824648e-3) * t33886;
    let t33894 = t7457 * t7458 * t535 * t7459;
    (t33869, t33874, t33876, t33883, t33884, t33887, t33894)
}
