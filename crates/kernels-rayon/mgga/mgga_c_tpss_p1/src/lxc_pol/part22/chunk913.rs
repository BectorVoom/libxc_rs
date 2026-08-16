//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 913/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk913(t242: f64, t2675: f64, t2725: f64, t2722: f64, t2732: f64, t2731: f64, t2458: f64, t45: f64, t2004: f64, t924: f64, t2685: f64, t2689: f64) -> (f64, f64, f64, f64, f64) {
    let t8434 = t242 * t2675 * t2725;
    let t8435 = t2722 * t8434;
    let t8438 = t242 * t2675 * t2732;
    let t8439 = t2731 * t8438;
    let t8443 = t2458 * t45;
    let t8444 = 1.0_f64 / t8443;
    let t8450 = t2004 * t924;
    let t8453 = t2685 * t2689;
    (t8435, t8439, t8444, t8450, t8453)
}
