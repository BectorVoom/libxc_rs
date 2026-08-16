//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1865;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta550(t10073: f64, t25937: f64, t7282: f64, t7506: f64, t26069: f64, t96255: f64, t2453: f64, t3908: f64, t7507: f64, t2435: f64, t26301: f64, t7289: f64, t96276: f64, t94589: f64, t96279: f64, t26265: f64, t9686: f64, t2098: f64, t4075: f64, t786: f64, t2103: f64, t47567: f64, t26261: f64, t40270: f64, t25920: f64, t26260: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96398, t96401, t96403, t96410, t96412) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1865(t10073, t25937, t7282, t7506, t26069, t96255, t2453, t3908, t7507, t2435, t26301, t7289, t96276);
        let (t96456, t96460, t96463, t96473, t96491, t96503) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1866(t94589, t96279, t26265, t9686, t2098, t4075, t786, t2103, t47567, t26261, t40270, t10073, t25920, t26260);
    (t96398, t96401, t96403, t96410, t96412, t96456, t96460, t96463, t96473, t96491, t96503)
}
