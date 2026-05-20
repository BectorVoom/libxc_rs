//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1048/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1048<F: Float>(t32477: F, t786: F, t7060: F, t7063: F, t31770: F, t31775: F, t31835: F, t31842: F, t31847: F, t31855: F, t32458: F, t32460: F, t32463: F, t32464: F, t32473: F, t32476: F) -> (F, F, F, F, F) {
    let t32478 = t786 * t32477;
    let t32480 = F::cast_from(0.14456046980341999104e-1_f64) * t32478 * t7060;
    let t32481 = t7063 * t32477;
    let t32483 = F::cast_from(0.25702851531048074406e-1_f64) * t32481 * t7060;
    let t32485 = -t32458 - F::cast_from(0.3718732920905101082e-3_f64) * t31835 + t32460 - F::cast_from(0.225875734067843736e-2_f64) * t31770 - F::cast_from(0.56468933516960933999e-3_f64) * t31775 - F::cast_from(0.11423947533020470523e1_f64) * t32463 * t32464 + F::cast_from(0.7437465841810202164e-3_f64) * t31842 + F::cast_from(0.14874931683620404328e-2_f64) * t31855 - t32473 + t32476 + t32480 - t32483 + F::cast_from(0.7437465841810202164e-3_f64) * t31847;
    (t32478, t32480, t32481, t32483, t32485)
}
