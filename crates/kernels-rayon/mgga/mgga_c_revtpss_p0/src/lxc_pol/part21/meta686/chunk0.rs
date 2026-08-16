//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2503/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2503(t12772: f64, t12780: f64, t3625: f64, t13052: f64, t13054: f64, t3172: f64, t11262: f64, t3711: f64, t3713: f64, t12657: f64, t1284: f64, t3624: f64) -> (f64, f64, f64, f64) {
    let t44729 = t3625 * t12772 * t12780;
    let t44748 = t13052 * t3172 * t13054;
    let t44751 = t3711 * t11262 * t3713;
    let t44769 = t12657 * t1284 * t3624;
    (t44729, t44748, t44751, t44769)
}
