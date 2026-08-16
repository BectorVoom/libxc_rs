//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1086/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1086(t38175: f64, t10841: f64, t1607: f64, t2228: f64, t505: f64, t539: f64, t10856: f64, t6245: f64, t252: f64, t3320: f64, t6262: f64, t783: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38176 = 0.174549769648958674e0_f64 * t38175;
    let t38177 = t10841 * t1607;
    let t38182 = t2228 * t505;
    let t38183 = t38182 * t539;
    let t38185 = t10856 * t6245;
    let t38189 = t783 * t252 * t6262 * t3320;
    (t38176, t38177, t38182, t38183, t38185, t38189)
}
