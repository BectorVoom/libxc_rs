//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1144/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1144(t23587: f64, t8375: f64, t23384: f64, t30905: f64, t30781: f64, t6698: f64, t1920: f64, t2966: f64, t8376: f64, t30869: f64, t30796: f64, t6680: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113278 = t8375 * t23587;
    let t113286 = t23384 * t30905;
    let t113296 = t6698 * t30781;
    let t113313 = 0.36554090374405031922e-2_f64 * t1920 * t2966 * t8376;
    let t113314 = t23384 * t30869;
    let t113318 = t6680 * t30796;
    (t113278, t113286, t113296, t113313, t113314, t113318)
}
