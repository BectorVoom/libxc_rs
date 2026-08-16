//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1508;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1509;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta399(t14676: f64, t4364: f64, t837: f64, t2646: f64, t4365: f64, t136: f64, t243: f64, t220: f64, t14671: f64, t10777: f64, t125: f64, t4343: f64, t2747: f64, t4450: f64, t10779: f64, t1548: f64, t10811: f64, t4447: f64, t2749: f64, t10673: f64, t10676: f64, t14668: f64, t14675: f64, t2745: f64, t4362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14678, t14682, t14685, t14686, t14688, t14690, t14691) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1508(t14676, t4364, t837, t2646, t4365, t136, t243, t220, t14671, t10777, t125, t4343);
        let (t14693, t14697, t14701, t14703, t14705, t14707) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1509(t14691, t2747, t837, t2646, t4450, t10779, t1548, t10777, t10811, t4447, t14676, t2749);
        let t14711 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1510(t10673, t10676, t14668, t14675, t14678, t14682, t14690, t14693, t14697, t14703, t14705, t14707, t2745, t4362);
    (t14678, t14682, t14685, t14686, t14688, t14693, t14697, t14701, t14707, t14711)
}
