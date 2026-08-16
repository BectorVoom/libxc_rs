//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1320/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1320(t13176: f64, t816: f64, t1512: f64, t9671: f64, t1484: f64, t2379: f64, t820: f64, t9607: f64, t2697: f64, t4257: f64, t4119: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13177 = t13176 * t816;
    let t13182 = t9671 * t1512;
    let t13184 = t1484 * t2379;
    let t13186 = t9607 * t820 * t13184;
    let t13190 = 35.0_f64 / 576.0_f64 * t2697 * t4257;
    let t13191 = t4119 * t776;
    (t13177, t13182, t13184, t13186, t13190, t13191)
}
