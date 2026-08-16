//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 888/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk888<F: Float>(t1717: F, t2630: F, t1646: F, t3073: F, t3074: F, t1728: F, t10170: F, t1727: F, t1030: F, t829: F, t1045: F, t10093: F, t10150: F, t10182: F, t10188: F, t10190: F, t10192: F, t10194: F, t1083: F, t13618: F, t13620: F, t13623: F, t13627: F, t13630: F, t1697: F, t278: F, t305: F, t3061: F, t3166: F, t4768: F, t4920: F, t975: F) -> F {
    let t13633 = t1717 * t2630;
    let t13636 = t3073 * t1646;
    let t13637 = t13636 * t3074;
    let t13640 = t1728 * t2630;
    let t13643 = t10170 * t1727;
    let t13644 = t13643 * t3074;
    let t13658 = t1030 * t3073;
    let t13659 = t1727 * t829;
    let t13660 = t13659 * t1045;
    let t13663 = -t278 * t13618 + F::cast_from(0.93706135855523581992e-2_f64) * t1030 * t13620 + F::cast_from(0.46853067927761790996e-2_f64) * t1030 * t13623 + F::cast_from(0.28111840756657074598e-1_f64) * t305 * t13627 + F::cast_from(0.14055920378328537299e-1_f64) * t305 * t13630 - F::cast_from(0.14055920378328537299e-1_f64) * t10093 * t13633 - F::cast_from(0.14055920378328537299e-1_f64) * t1030 * t13637 - F::cast_from(0.93706135855523581992e-2_f64) * t3061 * t13640 - F::cast_from(0.56223681513314149196e-1_f64) * t305 * t13644 - F::cast_from(2.0_f64) * t4768 * t1083 - t1697 * t3166 - F::cast_from(2.0_f64) * t975 * t4920 - F::cast_from(0.14055920378328537299e-1_f64) * t10150 - F::cast_from(0.46853067927761790996e-2_f64) * t10182 - F::cast_from(0.93706135855523581992e-2_f64) * t10188 - F::cast_from(0.18741227171104716398e-1_f64) * t10190 + F::cast_from(0.23426533963880895498e-2_f64) * t10192 + F::cast_from(0.46853067927761790996e-2_f64) * t10194 - F::cast_from(0.28111840756657074598e-1_f64) * t13658 * t13660;
    t13663
}
