//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1271/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1271<F: Float>(t1114: F, t332: F, t3747: F, t13793: F, t14617: F, t53229: F, t3060: F, t36200: F, t36201: F, t4155: F, t52902: F, t56061: F, t56063: F, t56067: F, t56070: F, t56074: F, t56077: F, t56080: F, t56084: F, t56093: F, t56098: F, t56101: F, t827: F) -> (F, F) {
    let t56104 = t1114 * t3747 * t332;
    let t56105 = t56104 * t13793;
    let t56107 = t53229 * t14617;
    let t56109 = t56061 / F::new(48.0) + F::new(7.0) / F::new(288.0) * t56063 + t56067 / F::new(384.0) + F::new(5.0) / F::new(384.0) * t56070 - t56074 / F::new(1536.0) - t56077 / F::new(192.0) - t56080 / F::new(192.0) - t827 * t56084 / F::new(96.0) - t52902 + t36200 * t36201 * t4155 * t3060 / F::new(4.0) - t56093 / F::new(96.0) - t56098 / F::new(384.0) - t56101 / F::new(48.0) - t56105 / F::new(48.0) + F::new(7.0) / F::new(144.0) * t56107;
    (t56104, t56109)
}
