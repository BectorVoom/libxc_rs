//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1246/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1246(t3685: f64, t5559: f64, t3689: f64, t1705: f64, t3692: f64, t935: f64, t5570: f64, t6134: f64) -> (f64, f64, f64, f64, f64) {
    let t19720 = t5559 * t3685;
    let t19722 = t5559 * t3689;
    let t19733 = t1705 * t3692;
    let t19734 = t19733 * t935;
    let t19736 = t6134 * t5570;
    (t19720, t19722, t19733, t19734, t19736)
}
