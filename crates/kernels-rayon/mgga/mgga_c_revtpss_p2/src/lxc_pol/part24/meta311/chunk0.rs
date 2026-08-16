//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1098/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1098(t3930: f64, t6846: f64, t221: f64, t4019: f64, t6862: f64, t10001: f64, t6800: f64, t72: f64, t757: f64, t1317: f64, t6801: f64, t1320: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22179 = t3930 * t6846;
    let t22182 = t4019 * t221 * t6862;
    let t22183 = t10001 * t22182;
    let t22185 = t6800 * t72;
    let t22186 = t22185 * t757;
    let t22188 = t1317 * t6801;
    let t22191 = t1320 * t6801;
    (t22179, t22182, t22183, t22185, t22186, t22188, t22191)
}
