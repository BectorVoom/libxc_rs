//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1834;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1835;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1836;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1837;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta433(t11150: f64, t5819: f64, t606: f64, t2850: f64, t128: f64, t4186: f64, t4573: f64, t6093: f64, t689: f64, t6097: f64, t6092: f64, t904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18908, t18909, t18910, t18911) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1834(t11150, t5819, t606, t2850, t128);
        let (t18913, t18914, t18915) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1835(t4186, t4573, t2850, t128);
        let t18919 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1836(t6093, t689);
        let t18924 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1837(t6097, t689);
        let (t18926, t18927, t18928) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1838(t606, t6092, t904, t128);
    (t18908, t18909, t18910, t18911, t18913, t18914, t18915, t18919, t18924, t18926, t18927, t18928)
}
