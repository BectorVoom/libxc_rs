//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1846;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1847;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta394(t12810: f64, t3629: f64, t3626: f64, t221: f64, t462: f64, t68: f64, t461: f64, t1209: f64, t3766: f64, t5330: f64, t1214: f64, t3603: f64, t3720: f64, t1250: f64, t12726: f64, t11772: f64, t3623: f64, t3717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12846, t12847, t12851, t12853, t12854, t12855) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1846(t12810, t3629, t3626, t221, t462, t68, t461, t1209, t3766, t5330);
        let (t12857, t12858, t12861, t12862, t12865) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1847(t1214, t3603, t12810, t3720, t1250, t12726, t11772, t3623);
        let t12866 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1848(t12865, t3717);
    (t12846, t12847, t12851, t12853, t12854, t12855, t12857, t12858, t12861, t12862, t12865, t12866)
}
