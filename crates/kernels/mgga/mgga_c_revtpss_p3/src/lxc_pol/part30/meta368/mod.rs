//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta368<F: Float>(t12916: F, t3722: F, t3718: F, t3172: F, t3590: F, t1247: F, t3612: F, t3610: F, t1260: F, t3666: F, t3713: F, t3711: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12917, t12918, t12941, t12942, t12948, t12949, t12956, t12959, t12960) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1395::<F>(t12916, t3722, t3718, t3172, t3590, t1247, t3612, t3610, t1260, t3666, t3713, t3711);
    (t12917, t12918, t12941, t12942, t12948, t12949, t12956, t12959, t12960)
}
