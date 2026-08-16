//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 741/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk741(t71375: f64, t2039: f64, t2244: f64, t270: f64, t638: f64, t2227: f64, t235: f64, t7190: f64, t7262: f64, t14696: f64, t7491: f64, t69674: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71376 = 0.1951603679568577289e-3_f64 * t71375;
    let t71380 = t638 * t2039 * t2244 * t270;
    let t71400 = t235 * t7190 * t2227;
    let t71404 = t235 * t7262 * t2227;
    let t71418 = t7491 * t14696;
    let t71419 = 0.30487649791575028314e-3_f64 * t71418;
    let t71429 = 0.11351689503877428609e-7_f64 * t69674;
    (t71376, t71380, t71400, t71404, t71419, t71429)
}
