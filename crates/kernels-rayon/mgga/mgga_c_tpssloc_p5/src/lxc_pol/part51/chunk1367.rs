//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1367/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1367(t115903: f64, t119891: f64, t115833: f64, t119883: f64, t119879: f64, t31688: f64, t33115: f64, t12571: f64, t31687: f64, t8515: f64, t115889: f64, t115907: f64, t119938: f64, t119944: f64, t119952: f64, t119965: f64, t121074: f64, t121081: f64, t121087: f64, t121094: f64, t121099: f64, t31019: f64, t31672: f64, t31675: f64, t31677: f64, t31681: f64, t31693: f64, t33560: f64, t33572: f64, t46104: f64, t8511: f64, t8512: f64) -> f64 {
    let t121102 = t115903 * t119891;
    let t121105 = t115833 * t119883;
    let t121108 = t115833 * t119879;
    let t121121 = t31688 * t33115;
    let t121124 = t12571 * t31687 * t8515;
    let t121126 = 5.0_f64 / 12.0_f64 * t31675 * t121074 - 5.0_f64 / 36.0_f64 * t31672 * t33572 - 5.0_f64 / 36.0_f64 * t8512 * t121081 - 5.0_f64 / 36.0_f64 * t8512 * t119952 - 5.0_f64 / 36.0_f64 * t8512 * t121087 + 5.0_f64 / 12.0_f64 * t31675 * t119938 - 5.0_f64 / 36.0_f64 * t8512 * t119944 + 5.0_f64 / 12.0_f64 * t121094 * t31677 - 5.0_f64 / 36.0_f64 * t33560 * t31693 + 5.0_f64 / 9.0_f64 * t31681 * t121099 + 5.0_f64 / 9.0_f64 * t31681 * t121102 - 5.0_f64 / 3.0_f64 * t115907 * t121105 - 5.0_f64 / 3.0_f64 * t115907 * t121108 - 20.0_f64 / 27.0_f64 * t115889 - 5.0_f64 / 72.0_f64 * t31672 * t33115 - 5.0_f64 / 72.0_f64 * t8512 * t119965 - 5.0_f64 / 72.0_f64 * t46104 * t8511 * t8515 - 5.0_f64 / 72.0_f64 * t33560 * t31019 + 5.0_f64 / 27.0_f64 * t121121 + 5.0_f64 / 27.0_f64 * t121124;
    t121126
}
