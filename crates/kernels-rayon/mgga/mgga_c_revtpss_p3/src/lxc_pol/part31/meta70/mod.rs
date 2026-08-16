//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta70 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk450;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk451;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta70(t1362: f64, t1364: f64, t535: f64, t795: f64, t159: f64, t540: f64, t216: f64, t124: f64, t1353: f64, t800: f64, t546: f64, t550: f64, t808: f64, t807: f64, t547: f64, t786: f64, t814: f64, t816: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1366, t1368, t1369, t1370) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk450(t1362, t1364, t535, t795, t159, t540, t216);
        let t1372 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk451(t124, t1353, t800);
        let (t1376, t1378, t1379, t1381, t1383, t1384, t1385) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk452(t546, t550, t808, t807, t547, t786, t814, t816, t544);
    (t1366, t1368, t1369, t1370, t1372, t1376, t1378, t1379, t1381, t1383, t1384, t1385)
}
