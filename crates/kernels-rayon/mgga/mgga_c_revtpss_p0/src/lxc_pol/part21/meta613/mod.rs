//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2365;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta613(t10468: f64, t750: f64, t10555: f64, t10605: f64, t10436: f64, t2398: f64, t10356: f64, t10439: f64, t10565: f64, t717: f64, t10587: f64, t2496: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t40141, t40143, t40145, t40148, t40150, t40156) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2365(t10468, t750, t10555, t10605, t10436, t2398, t10356, t10439, t10565, t717, t10587, t2496);
    (t40141, t40143, t40145, t40148, t40150, t40156)
}
