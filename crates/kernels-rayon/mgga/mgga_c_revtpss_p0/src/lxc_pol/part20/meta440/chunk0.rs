//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1671/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1671(t3555: f64, t3727: f64, t13180: f64, t493: f64, t225: f64, t3738: f64, t3790: f64, t1209: f64, t13107: f64, t460: f64, t1269: f64, t13043: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45545 = t3555 * t3727;
    let t45551 = 1.0_f64 / t13180 / t493;
    let t45552 = t225 * t45551;
    let t45553 = t3738 * t3738;
    let t45559 = t3790 * t3790;
    let t45568 = t1209 * t13107;
    let t45575 = t460 * t13107;
    let t45584 = t1269 * t13043;
    (t45545, t45552, t45553, t45559, t45568, t45575, t45584)
}
