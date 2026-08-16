//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1352/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1352(t1692: f64, t1812: f64, t7622: f64, t555: f64, t1288: f64, t18728: f64, t18803: f64, t18812: f64, t19681: f64, t19685: f64, t19810: f64, t20417: f64, t20526: f64, t2439: f64, t35530: f64, t5849: f64, t5853: f64, t6153: f64, t62610: f64, t62820: f64, t6331: f64, t63794: f64, t63797: f64, t63817: f64, t63860: f64, t63881: f64, t64249: f64, t64292: f64) -> (f64, f64, f64) {
    let t66631 = 3.0_f64 * t1692 * t1812 * t7622;
    let t66641 = t1692 * t1812 * t555;
    let t66656 = -t1692 * t62820 * t6153 / 2.0_f64 + t1692 * t18803 * t1288 / 2.0_f64 + 3.0_f64 * t35530 * t6331 - 6.0_f64 * t20417 * t63860 - t66631 + 3.0_f64 * t18728 * t63797 + 3.0_f64 * t2439 * t5849 * t19685 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t64292 + t66641 - 3.0_f64 * t20526 * t64249 + t1692 * t18812 * t63794 - t1692 * t5853 * t63817 / 2.0_f64 + 3.0_f64 * t2439 * t5849 * t19681 - 3.0_f64 * t62610 * t19810 - 3.0_f64 * t18728 * t63881;
    (t66631, t66641, t66656)
}
