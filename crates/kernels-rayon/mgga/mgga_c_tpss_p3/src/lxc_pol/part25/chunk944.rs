//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 944/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk944(t11687: f64, t946: f64, t1407: f64, t242: f64, t8951: f64, t967: f64, t2748: f64, t3969: f64, t2675: f64, t3950: f64, t219: f64, t3988: f64) -> (f64, f64, f64, f64, f64) {
    let t11688 = t946 * t11687;
    let t11691 = t242 * t8951 * t1407;
    let t11692 = t967 * t11691;
    let t11697 = t2748 * t3969 / 648.0_f64;
    let t11701 = t242 * t2675 * t3950;
    let t11703 = t946 * t11701 / 2304.0_f64;
    let t11710 = t3988 * t219;
    (t11688, t11692, t11697, t11703, t11710)
}
