//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1116/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1116(t1501: f64, t3055: f64, t3068: f64, t11821: f64, t11823: f64, t11828: f64, t11832: f64, t11836: f64, t11839: f64, t11970: f64, t11973: f64, t11975: f64, t11978: f64, t11980: f64, t11982: f64, t12002: f64, t12004: f64, t12006: f64, t12008: f64, t12011: f64) -> (f64, f64) {
    let t12329 = t1501 * t3055;
    let t12330 = t3068 * t12329;
    let t12333 = -t11821 + t11823 - t11828 + t11832 - t11836 - t11839 + t11970 + t11973 + t11975 + t11978 + t11980 + t11982 + t12002 - t12004 + t12006 - t12008 - t12011;
    (t12330, t12333)
}
