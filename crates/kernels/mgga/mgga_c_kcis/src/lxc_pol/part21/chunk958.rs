//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 958/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk958<F: Float>(t291: F, t4951: F, t13511: F, t14538: F, t14543: F, t14548: F, t14551: F, t2872: F, t4963: F, t9883: F, t9906: F, t991: F, t9910: F, t9918: F, t9940: F, t9957: F, t9961: F, t9970: F) -> (F, F) {
    let t14554 = t4951 * t291;
    let t14555 = t14554 * t13511;
    let t14561 = t9883 - t9906 / F::new(162.0) - t9910 / F::new(432.0) - t9918 / F::new(648.0) - t9940 / F::new(432.0) - t14538 + t2872 * t4963 / F::new(54.0) + t991 * t14543 / F::new(144.0) - t991 * t14548 / F::new(72.0) - t991 * t14551 / F::new(144.0) - t991 * t14555 / F::new(36.0) + t9957 / F::new(864.0) + t9961 / F::new(648.0) + t9970 / F::new(81.0);
    (t14554, t14561)
}
