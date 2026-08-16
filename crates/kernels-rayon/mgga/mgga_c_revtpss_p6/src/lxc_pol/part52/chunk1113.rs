//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1113/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1113(t2411: f64, t28455: f64, t198: f64, t206: f64, t8019: f64, t2718: f64, t7398: f64, t41040: f64, t685: f64, t10867: f64, t2061: f64, t11064: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102854 = t28455 * t2411;
    let t102888 = t198 * t206 * t8019;
    let t103059 = t2718 * t7398;
    let t103181 = t685 * t41040;
    let t103452 = t10867 * t2061;
    let t103586 = t8019 * t11064;
    (t102854, t102888, t103059, t103181, t103452, t103586)
}
