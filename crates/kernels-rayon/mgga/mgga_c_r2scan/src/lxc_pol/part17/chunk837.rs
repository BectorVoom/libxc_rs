//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 837/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk837(t2667: f64, t2727: f64, t2177: f64, t3198: f64, t1632: f64, t3190: f64, t551: f64, t2184: f64, t2892: f64, t2196: f64, t3158: f64, t378: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8861 = t2667 * t2727;
    let t8863 = t2177 * t3198;
    let t8865 = t1632 * t3190;
    let t8866 = t551 * t8865;
    let t8867 = t2184 * t8866;
    let t8872 = t1632 * t2892;
    let t8873 = t551 * t8872;
    let t8874 = t2196 * t8873;
    let t8879 = t5 * t378 * t3158;
    (t8861, t8863, t8865, t8867, t8872, t8874, t8879)
}
