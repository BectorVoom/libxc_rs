//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1875;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1876;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta518(t265: f64, t393: f64, t1100: f64, t1102: f64, t1699: f64, t198: f64, t25709: f64, t25713: f64, t27708: f64, t27712: f64, t27717: f64, t27754: f64, t336: f64, t5019: f64, t5023: f64, t7181: f64, t30: f64, t1469: f64, t1996: f64, t27408: f64, t4186: f64, t45: f64, t606: f64, t7194: f64, t7856: f64, t33: f64, t892: f64, t4433: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t18875: f64, t25759: f64, t1113: f64, t1544: f64, t4343: f64, t27375: f64, t11064: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t27755 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1875(t265, t393, t1100, t1102, t1699, t198, t25709, t25713, t27708, t27712, t27717, t27754, t336, t5019, t5023, t7181);
        let (t27762, t27763, t27764) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1876(t30, t1469, t1996, t27408, t27755, t4186, t45, t606, t7194, t7856, t33, t892, t4433, dens_threshold, rho0, zeta_threshold);
        let (t27770, t27773, t27777, t27793, t27799) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1877(t18875, t25759, t1113, t1544, t33, t4343, t27375, t11064);
    (t27755, t27762, t27763, t27764, t27770, t27773, t27777, t27793, t27799)
}
