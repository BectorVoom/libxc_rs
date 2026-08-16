//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta69 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk417;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk418;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk419;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta69<F: Float>(t1362: F, t1364: F, t535: F, t795: F, t159: F, t540: F, t216: F, t124: F, t1353: F, t800: F, t546: F, t550: F, t808: F, t807: F, t547: F, t786: F, t814: F, t816: F, t544: F, t235: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1366, t1368, t1369, t1370) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk417::<F>(t1362, t1364, t535, t795, t159, t540, t216);
        let (t1372, t1376, t1378, t1379, t1383, t1384) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk418::<F>(t124, t1353, t800, t546, t550, t808, t807, t547, t786, t814, t816, t544);
        let t1385 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk419::<F>(t1384);
        let t1386 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk420::<F>(t1385, t235);
    (t1366, t1368, t1369, t1370, t1372, t1376, t1378, t1379, t1383, t1384, t1385, t1386)
}
