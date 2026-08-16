//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1463/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1463(t3205: f64, t371: f64, t6337: f64, t676: f64, t15731: f64, t4879: f64, t225: f64, t64686: f64, t366: f64, t19566: f64, t3090: f64, t1086: f64, t19462: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67206 = t3205 * t371 * t676 * t6337;
    let t67473 = t4879 * t15731;
    let t67501 = t64686 * t225;
    let t67502 = t67501 * t366;
    let t67528 = t19566 * t3090;
    let t67551 = t19462 * t1086 * t3090;
    (t67206, t67473, t67501, t67502, t67528, t67551)
}
