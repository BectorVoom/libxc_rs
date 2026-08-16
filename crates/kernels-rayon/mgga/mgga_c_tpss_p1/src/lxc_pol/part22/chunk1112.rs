//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1112/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1112(t12078: f64, t12175: f64, t12217: f64, t12273: f64, t294: f64, t1151: f64, t11821: f64, t11823: f64, t11828: f64, t11832: f64, t11836: f64, t11839: f64, t11970: f64, t11973: f64, t11975: f64, t11978: f64, t11980: f64, t11982: f64, t12002: f64, t12004: f64, t12006: f64, t12008: f64, t12011: f64, t12012: f64, t4023: f64) -> (f64, f64) {
    let t12276 = t294 * (t12078 + t12175 + t12217 + t12273);
    let t12277 = -2.0_f64 * t1151 * t12012 * t4023 - t11821 + t11823 - t11828 + t11832 - t11836 - t11839 + t11970 + t11973 + t11975 + t11978 + t11980 + t11982 + t12002 - t12004 + t12006 - t12008 - t12011 + t12276;
    (t12276, t12277)
}
