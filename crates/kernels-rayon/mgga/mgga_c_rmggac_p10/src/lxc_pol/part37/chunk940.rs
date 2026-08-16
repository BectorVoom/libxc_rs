//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 940/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk940(t2080: f64, t739: f64, t9530: f64, t2010: f64, t2012: f64, t9343: f64, t637: f64, t8901: f64, t71167: f64, t8905: f64, t71007: f64, t8621: f64) -> (f64, f64, f64, f64, f64) {
    let t77036 = t739 * t9530 * t2080;
    let t77037 = 0.2993560425465952141e-1_f64 * t77036;
    let t77042 = t2010 * t2012 * t9343;
    let t77043 = 0.36021158228745895953e-3_f64 * t77042;
    let t77044 = t637 * t8901;
    let t77045 = t71167 * t77044;
    let t77046 = 0.20455996240684006297e-1_f64 * t77045;
    let t77047 = t637 * t8905;
    let t77048 = t71007 * t77047;
    let t77049 = 0.27274661654245341729e-1_f64 * t77048;
    let t77050 = t637 * t8621;
    (t77037, t77043, t77046, t77049, t77050)
}
