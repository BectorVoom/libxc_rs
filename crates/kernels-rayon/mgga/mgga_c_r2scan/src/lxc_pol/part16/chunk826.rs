//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 826/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk826(t1541: f64, t3016: f64, t481: f64, t490: f64, t8629: f64, t109: f64, t111: f64, t2498: f64, t2504: f64, t2506: f64, t2527: f64, t3042: f64, t3046: f64, t3049: f64, t486: f64, t491: f64, t8662: f64, t8668: f64, t8676: f64, t8679: f64, t915: f64, t917: f64) -> f64 {
    let t8684 = t1541 * t3016;
    let t8685 = t8684 * t481;
    let t8688 = t490 * t8629;
    let t8691 = 3.0_f64 * t109 * t8688 - t8662 * t111 + 6.0_f64 * t2498 * t917 + 60.0_f64 * t2504 * t8676 - 24.0_f64 * t2504 * t8679 - 12.0_f64 * t2504 * t8685 - 24.0_f64 * t8668 * t2506 + 6.0_f64 * t915 * t2527 + 3.0_f64 * t3042 * t491 - 12.0_f64 * t486 * t3046 + 3.0_f64 * t486 * t3049;
    t8691
}
