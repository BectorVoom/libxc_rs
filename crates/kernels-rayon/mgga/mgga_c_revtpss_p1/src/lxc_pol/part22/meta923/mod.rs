//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta923 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta923(t12627: f64, t1811: f64, t12657: f64, t1208: f64, t17330: f64, t487: f64, t1269: f64, t17306: f64, t1209: f64, t1270: f64, t3566: f64, t17331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56393, t56396, t56412, t56413, t56416, t56419, t56432, t56486) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3145(t12627, t1811, t12657, t1208, t17330, t487, t1269, t17306, t1209, t1270, t3566, t17331);
    (t56393, t56396, t56412, t56413, t56416, t56419, t56432, t56486)
}
