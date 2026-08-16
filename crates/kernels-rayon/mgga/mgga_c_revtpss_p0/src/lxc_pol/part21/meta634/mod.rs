//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2404;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2405;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta634(t11003: f64, t9303: f64, t10978: f64, t689: f64, t779: f64, t10981: f64, t22: f64, t868: f64, t886: f64, t10910: f64, t212: f64, t780: f64, t10988: f64, t2435: f64, t2445: f64, t9292: f64, t11025: f64, t588: f64, t10991: f64, t39497: f64, t787: f64, t788: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40970, t40973, t40978, t40982) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2404(t11003, t9303, t10978, t689, t779, t10981, t22, t868, t886, t10910, t212, t780);
        let (t40986, t40988, t40994, t40998, t40999, t41003) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2405(t10988, t2435, t2445, t9292, t11025, t10981, t588, t780, t10991, t39497, t787, t788);
    (t40970, t40973, t40978, t40982, t40986, t40988, t40994, t40998, t40999, t41003)
}
