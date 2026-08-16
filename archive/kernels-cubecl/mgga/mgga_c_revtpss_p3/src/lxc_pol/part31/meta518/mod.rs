//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1875;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1876;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta518<F: Float>(t265: F, t393: F, t1100: F, t1102: F, t1699: F, t198: F, t25709: F, t25713: F, t27708: F, t27712: F, t27717: F, t27754: F, t336: F, t5019: F, t5023: F, t7181: F, t30: F, t1469: F, t1996: F, t27408: F, t4186: F, t45: F, t606: F, t7194: F, t7856: F, t33: F, t892: F, t4433: F, dens_threshold: F, rho0: F, zeta_threshold: F, t18875: F, t25759: F, t1113: F, t1544: F, t4343: F, t27375: F, t11064: F) -> (F, F, F, F, F, F, F, F, F) {
        let t27755 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1875::<F>(t265, t393, t1100, t1102, t1699, t198, t25709, t25713, t27708, t27712, t27717, t27754, t336, t5019, t5023, t7181);
        let (t27762, t27763, t27764) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1876::<F>(t30, t1469, t1996, t27408, t27755, t4186, t45, t606, t7194, t7856, t33, t892, t4433, dens_threshold, rho0, zeta_threshold);
        let (t27770, t27773, t27777, t27793, t27799) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1877::<F>(t18875, t25759, t1113, t1544, t33, t4343, t27375, t11064);
    (t27755, t27762, t27763, t27764, t27770, t27773, t27777, t27793, t27799)
}
