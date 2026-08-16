//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta855 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2999;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta855(t2829: f64, t4321: f64, t689: f64, t15054: f64, t786: f64, t789: f64, t2465: f64, t4480: f64, t9288: f64, t1569: f64, t2769: f64, t10997: f64, t10985: f64, t15017: f64, t15045: f64, t2435: f64, t15048: f64, t2471: f64, t15008: f64, t10996: f64, t14990: f64, t41070: f64, t14939: f64, t212: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50198, t50201, t50205, t50208, t50209) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2999(t2829, t4321, t689, t15054, t786, t789, t2465, t4480, t9288, t1569, t2769, t10997);
        let (t50214, t50218, t50220, t50222, t50227, t50232) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3000(t10985, t15017, t15045, t2435, t15048, t2471, t15008, t10996, t14990, t41070, t14939, t212, t689, t780);
    (t50198, t50201, t50205, t50208, t50209, t50214, t50218, t50220, t50222, t50227, t50232)
}
