//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta363(t14685: f64, t220: f64, t14671: f64, t837: f64, t10777: f64, t10779: f64, t1548: f64, t10811: f64, t4447: f64, t10815: f64, t1561: f64, t2741: f64, t4426: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14686, t14688, t14690, t14701, t14703, t14705, t14712, t14715) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1392(t14685, t220, t14671, t837, t10777, t10779, t1548, t10811, t4447, t10815, t1561, t2741, t4426);
    (t14686, t14688, t14690, t14701, t14703, t14705, t14712, t14715)
}
