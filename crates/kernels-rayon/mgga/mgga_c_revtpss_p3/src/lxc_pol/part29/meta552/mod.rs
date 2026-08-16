//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1890;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta552(t26265: f64, t9671: f64, t26230: f64, t94403: f64, t25904: f64, t4078: f64, t689: f64, t7492: f64, t94589: f64, t96279: f64, t25895: f64, t96239: f64, t9686: f64, t2098: f64, t4075: f64, t786: f64, t9682: f64, t2103: f64, t47567: f64, t1364: f64, t26338: f64, t26261: f64, t40270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96423, t96431, t96432, t96437, t96456, t96458) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1890(t26265, t9671, t26230, t94403, t25904, t4078, t689, t7492, t94589, t96279, t25895, t96239);
        let (t96460, t96463, t96464, t96473, t96486, t96491) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1891(t26265, t9686, t2098, t4075, t786, t9682, t2103, t47567, t1364, t26338, t26261, t40270);
    (t96423, t96431, t96432, t96437, t96456, t96458, t96460, t96463, t96464, t96473, t96486, t96491)
}
