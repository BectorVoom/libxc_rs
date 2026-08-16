//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3018/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3018(t2710: f64, t4371: f64, t9732: f64, t10886: f64, t14833: f64, t808: f64, t10811: f64, t14793: f64, t14774: f64, t2652: f64, t10726: f64, t14860: f64, t2661: f64, t4366: f64) -> (f64, f64, f64, f64, f64) {
    let t50703 = t2710 * t9732 * t4371;
    let t50706 = t10886 * t808 * t14833;
    let t50722 = t10811 * t14793;
    let t50724 = t2652 * t14774;
    let t50728 = t2661 * t10726 * t14860 * t4366;
    (t50703, t50706, t50722, t50724, t50728)
}
