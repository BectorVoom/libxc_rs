//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2584/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2584(t45619: f64, t58919: f64, t3666: f64, t5390: f64, t43766: f64, t44361: f64, t45608: f64, t45786: f64, t12984: f64, t5323: f64, t17500: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t58920 = t45619 * t58919;
    let t58927 = t3666 * t5390;
    let t58983 = t44361 * t43766;
    let t59001 = t45608 * t58919;
    let t59011 = t45786 * t58919;
    let t59040 = t5323 * t12984;
    let t59041 = 0.7622047665434619906e-3_f64 * t59040;
    let t59062 = t372 * t17500;
    (t58920, t58927, t58983, t59001, t59011, t59041, t59062)
}
