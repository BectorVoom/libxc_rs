//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1134/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1134<F: Float>(t119757: F, t31846: F, t4461: F, t119752: F, t4446: F, t119751: F, t33714: F, t837: F, t119783: F, t4365: F, t1579: F, t775: F) -> (F, F, F, F, F) {
    let t126065 = t31846 * t119757 * t4461;
    let t126068 = t31846 * t119752 * t4446;
    let t126072 = t119751 * t119752 * t33714 * t837;
    let t126076 = t119751 * t119752 * t4365 * t119783;
    let t126078 = t1579 * t775;
    (t126065, t126068, t126072, t126076, t126078)
}
