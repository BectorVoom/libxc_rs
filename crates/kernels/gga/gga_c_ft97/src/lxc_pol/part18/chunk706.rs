//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 706/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk706<F: Float>(t11484: F, t11546: F, t11591: F, t11822: F, t11876: F, t11910: F, t11997: F, t12055: F, t103: F, t11801: F, t1578: F, t649: F, t1073: F, t1559: F, t1570: F, t2266: F) -> (F, F, F, F) {
    let t12058 = t11484 + t11546 + t11591 + t11822 + t11876 + t11910 + t11997 + t12055;
    let t12062 = t11801 * t103;
    let t12092 = t649 * t1578;
    let t12099 = t2266 * t1073 * t1570 * t1559;
    (t12058, t12062, t12092, t12099)
}
