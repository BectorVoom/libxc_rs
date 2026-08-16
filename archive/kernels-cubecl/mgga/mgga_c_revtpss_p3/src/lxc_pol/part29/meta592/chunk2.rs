//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1969/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1969<F: Float>(t2435: F, t8099: F, t25904: F, t26231: F, t97802: F, t26234: F, t98041: F, t102244: F, t94674: F, t97700: F, t102268: F, t1882: F, t25921: F, t25930: F, t26335: F, t28863: F, t28890: F, t28911: F, t7292: F, t7917: F, t96296: F, t96298: F, t96371: F, t96374: F, t96378: F, t98362: F) -> (F, F) {
    let t102315 = t8099 * t2435;
    let t102316 = t25904 * t102315;
    let t102320 = F::cast_from(0.14456046980341999104e-1_f64) * t97802 * t26231;
    let t102324 = F::cast_from(0.51405703062096148812e-1_f64) * t98041 * t26234;
    let t102325 = t94674 * t102244;
    let t102329 = F::cast_from(0.28912093960683998208e-1_f64) * t97700 * t26234;
    let t102339 = F::cast_from(0.14456046980341999104e-1_f64) * t25904 * t102268;
    let t102341 = -F::cast_from(0.28912093960683998208e-1_f64) * t96296 + F::cast_from(0.96373646535613327357e-2_f64) * t102316 + F::cast_from(0.19274729307122665471e-1_f64) * t96298 - t102320 - F::cast_from(0.4336814094102599731e0_f64) * t7917 * t26335 + t102324 + F::cast_from(0.86736281882051994623e-1_f64) * t102325 - F::cast_from(0.12851425765524037203e-1_f64) * t96371 + t96374 - t102329 + F::cast_from(0.34694512752820797848e1_f64) * t25930 * t28911 * t1882 * t98362 + F::cast_from(0.17347256376410398924e1_f64) * t25921 * t28863 - F::cast_from(0.8673628188205199462e0_f64) * t7292 * t28890 - t102339 - F::cast_from(0.77108554593144223218e-1_f64) * t96378;
    (t102315, t102341)
}
