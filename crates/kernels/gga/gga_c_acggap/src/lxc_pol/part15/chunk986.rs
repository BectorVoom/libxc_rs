//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 986/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk986<F: Float>(t30717: F, t1998: F, t4625: F, t1434: F, t7736: F, t1418: F, t7614: F, t1083: F, t1487: F, t1980: F, t355: F, t7458: F) -> (F, F, F, F, F) {
    let t34743 = F::new(35.0) / F::new(108.0) * t30717;
    let t34745 = t1998 * t4625;
    let t34751 = t7736 * t1434;
    let t34753 = t7614 * t1418;
    let t34767 = t1980 * t7458 * t1083 * t355 * t1487;
    (t34743, t34745, t34751, t34753, t34767)
}
