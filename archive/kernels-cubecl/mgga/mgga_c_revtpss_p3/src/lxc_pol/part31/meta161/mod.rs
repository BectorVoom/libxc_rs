//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk808;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta161<F: Float>(t1843: F, t670: F, t2616: F, t2524: F, t1534: F, t72: F, t757: F, t1469: F, t750: F, t706: F, t190: F, t4186: F, t1531: F, t705: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4297, t4300, t4301, t4302, t4303, t4304, t4305, t4306, t4307, t4308) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk808::<F>(t1843, t670, t2616, t2524, t1534, t72, t757, t1469, t750, t706, t190, t4186);
        let (t4310, t4311) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk809::<F>(t4308, t706, t1531, t705);
    (t4297, t4300, t4301, t4302, t4303, t4304, t4305, t4306, t4307, t4308, t4310, t4311)
}
