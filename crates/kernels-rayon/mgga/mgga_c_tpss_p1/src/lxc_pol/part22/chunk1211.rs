//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1211/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1211(t1675: f64, t18660: f64, t1791: f64, t18331: f64, t5784: f64, t7690: f64, t38: f64, t599: f64, t1981: f64) -> (f64, f64, f64, f64, f64) {
    let t18661 = t1675 * t18660;
    let t18663 = t1791 * t18331;
    let t18666 = t7690 * t5784;
    let t18669 = t38 * t599;
    let t18670 = t1981 * t18669;
    (t18661, t18663, t18666, t18669, t18670)
}
