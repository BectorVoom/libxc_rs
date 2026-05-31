//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 345/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk345<F: Float>(t1672: F, t525: F, t1222: F, t515: F, t476: F, t508: F, t520: F, t10: F, t1240: F, t437: F, t440: F) -> (F, F, F, F, F, F, F, F) {
    let t1674 = F::cast_from(6.211752672544321_f64) * t525 * t1672;
    let t1675 = t1222 * t515;
    let t1677 = F::cast_from(0.013716887843283197_f64) * t476 * t1675;
    let t1679 = F::cast_from(1.6457779058161184_f64) * t508 * t1672;
    let t1680 = t520 * t520;
    let t1681 = F::cast_from(1.0_f64) / t1680;
    let t1683 = t1240 * t437 * t10;
    let t1684 = t1683 * t440;
    (t1674, t1675, t1677, t1679, t1680, t1681, t1683, t1684)
}
