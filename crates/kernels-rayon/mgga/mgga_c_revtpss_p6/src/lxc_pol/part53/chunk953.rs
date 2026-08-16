//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 953/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk953(t1502: f64, t1911: f64, t2007: f64, t2011: f64, t28175: f64, t28179: f64, t28183: f64, t28186: f64, t28188: f64, t28190: f64, t28192: f64, t28193: f64, t28201: f64, t28202: f64, t28230: f64, t4246: f64, t569: f64, t5787: f64, t7221: f64, t7231: f64) -> f64 {
    let t28232 = -t1502 * t7221 + t1911 * t7231 - t2007 * t4246 + t2011 * t5787 + t28230 * t569 + t28175 + t28179 - t28183 + t28186 - t28188 - t28190 + t28192 - t28193 + t28201 - t28202;
    t28232
}
