//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 739/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk739(t1289: f64, t2459: f64, t581: f64, t2457: f64, t128: f64, t2464: f64, t835: f64, t3431: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3748 = t2459 * t1289;
    let t3749 = t3748 * t581;
    let t3750 = t2457 * t3749;
    let t3751 = t128 * t3750;
    let t3753 = t2464 * t1289;
    let t3754 = t3753 * t581;
    let t3755 = t835 * t3754;
    let t3756 = t128 * t3755;
    let t3758 = t836 * t3431;
    (t3748, t3749, t3750, t3751, t3753, t3754, t3755, t3756, t3758)
}
